mod context;
mod controller;
mod model;
mod repository;
mod secrets;
mod settings;

use crate::{context::AppContext, settings::Settings};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web_opentelemetry::RequestTracing;
use common::{
    client::{
        cache_redis::{self, Cache, CachePool},
        db_mongo,
    },
    error::REDACTED_ERRORS,
    util::{actix_json_config::json_extractor_config, telemetry},
};
use secrets::Secrets;
use std::sync::Arc;
use tracing_actix_web::TracingLogger;

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

    let db_client = db_mongo::connect(app_name, &configuration.db, &secrets.db)
        .await
        .expect("db client connection failure");

    let cache_pool: CachePool = cache_redis::connect(&configuration.cache, &secrets.cache)?;
    let cache_client: Cache = cache_redis::Cache::new(cache_pool);

    // Instantiate the application context. This application state will be
    // cloned for each Actix thread but the Arc of the DbContext will be
    // reused in each Actix thread.
    let app_context = web::Data::new(AppContext {
        db: Arc::new(db_client),
        cache: Arc::new(cache_client),
    });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_context.clone())
            .app_data(json_extractor_config(4096))
            .wrap(TracingLogger::default())
            .wrap(RequestTracing::new())
            .service(web::scope("/user/v1.0").configure(controller::global_router))
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
