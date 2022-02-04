use crate::{controllers,
            settings::Settings};
use actix_governor::{Governor,
                     GovernorConfigBuilder};
use actix_web::{web,
                App,
                HttpResponse,
                HttpServer,
                Responder};
use actix_web_opentelemetry::RequestTracing;
use tracing_actix_web::TracingLogger;

pub async fn start_web_service(configuration: Settings) -> Result<(), std::io::Error> {
    let address = format!("{}:{}",
                          configuration.application.host, configuration.application.port);

    let governor_conf =
        GovernorConfigBuilder::default().per_second(configuration.rate_limit.frequency)
                                        .burst_size(configuration.rate_limit.burst_size)
                                        .finish()
                                        .unwrap();

    let server = HttpServer::new(move || {
                     App::new()
            .wrap(TracingLogger::default())
            .wrap(RequestTracing::new())
            // Enable Governor middleware
            .wrap(Governor::new(&governor_conf))
            .service(web::scope("/api/v1.0").configure(controllers::global_router))
            .default_service(web::get().to(not_found))
                 }).bind(&address)?;

    server.workers(configuration.application.workers)
          .run()
          .await?;

    Ok(())
}

pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("the requested resource does not exist")
}
