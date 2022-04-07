mod actor;
mod context;
mod controller;
mod model;
mod repository;
mod secrets;
mod settings;

use crate::{context::AppContext, settings::Settings};
use actix::Actor;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web_opentelemetry::RequestTracing;
use actor::{email_sender::EmailSender, event_stream_handler::EventStreamHandler};
use common::{
    client::cache_redis::{self, Cache, CachePool},
    error::REDACTED_ERRORS,
    model::event::v1::{auth::prelude::SERVICE_AUTH_SUBJECT, Event},
    util::{actix_json_config::json_extractor_config, telemetry},
};
use lettre::{
    transport::smtp::{authentication::Credentials, PoolConfig},
    SmtpTransport,
};
use nats_actor::subscriber::{NatsStreamMessage, NatsSubscriberConfig};
use secrecy::ExposeSecret;
use secrets::Secrets;
use std::sync::Arc;
use tracing::info;
use tracing_actix_web::TracingLogger;

use nats_actor::{subscriber::subscribe_to_nats, NatsClientSettings};

pub async fn server() -> Result<(), std::io::Error> {
    // configure tracing subscriber
    let app_name = concat!(env!("CARGO_PKG_NAME"), "-", env!("CARGO_PKG_VERSION")).to_string();

    let settings = Settings::load().expect("Failed to read configuration.");
    *REDACTED_ERRORS.write() = settings.log.redacted_errors;

    let jaeger_url = format!(
        "{}:{}",
        &settings.tracer.jaeger.host, &settings.tracer.jaeger.port,
    );
    telemetry::config_telemetry(&app_name, &jaeger_url);

    tracing::info!("services starting...");

    // Start Web server
    start_web_service(&app_name, settings).await?;
    // Ensure all spans have been reported
    opentelemetry::global::shutdown_tracer_provider();
    Ok(())
}

pub async fn start_web_service(
    app_name: &str,
    configuration: Settings,
) -> Result<(), std::io::Error> {
    let this_server_address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    let secrets: Secrets = secrets::read(&configuration).await?;

    let cache_pool: CachePool = cache_redis::connect(&configuration.cache, &secrets.cache)?;
    let cache_client: Cache = cache_redis::Cache::new(cache_pool);

    // Open a remote connection pool to SMTP server
    let smtp_mailer = SmtpTransport::relay(&configuration.smtp.server)
        .expect("failed to initialize SMTP client")
        .credentials(Credentials::new(
            secrets.smtp.user_name,
            secrets.cache.password.expose_secret().to_string(),
        ))
        .pool_config(
            PoolConfig::new()
                .min_idle(configuration.smtp.min_idle_connections)
                .max_size(configuration.smtp.max_pooled_connections)
                .idle_timeout(configuration.smtp.idle_timeout),
        )
        .build();

    // Start mail sendor actor
    let email_sender = EmailSender {
        smtp_mailer: Arc::new(smtp_mailer),
    }
    .start()
    .recipient();

    // Instantiate the application context. This application state will be
    // cloned for each Actix thread but the Arc of the DbContext will be
    // reused in each Actix thread.
    let app_context = web::Data::new(AppContext {
        cache: Arc::new(cache_client),
    });

    let nats_stream_handler = EventStreamHandler {
        context: Arc::clone(&app_context),
        email_sender,
    }
    .start();

    // start NATS subscriber for event streams
    actix::spawn(async move {
        subscribe_to_nats(
            NatsSubscriberConfig {
                client_settings: NatsClientSettings {
                    addresses: configuration.nats.addresses,
                    max_reconnects: configuration.nats.max_reconnects,
                    retry_timeout: configuration.nats.retry_timeout,
                },
                subject: SERVICE_AUTH_SUBJECT.into(),
                mailbox_size: configuration.application.nats_subscriber_mailbox_size,
            },
            move |msg: NatsStreamMessage| {
                info!("Received event {:?}", msg);
                let event: cloudevents::Event = serde_json::from_slice(&msg.msg.data).unwrap();
                let event: Event = event.try_into().unwrap();
                info!("Extracted event {:?}", event);
                nats_stream_handler.try_send(event);
                Ok(())
            },
        )
        .await
        .expect("nats connection/subscriber setup failure");
    });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_context.clone())
            .app_data(json_extractor_config(4096))
            .wrap(TracingLogger::default())
            .wrap(RequestTracing::new())
            .service(web::scope("/notification/v1.0").configure(controller::global_router))
            .default_service(web::get().to(not_found))
    })
    .bind(&this_server_address)?;

    server
        .workers(configuration.application.workers)
        .run()
        .await?;

    Ok(())
}

pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("the requested resource does not exist")
}
