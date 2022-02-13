use crate::{context::AppContext, controller, settings::Settings};
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web_opentelemetry::RequestTracing;
use common::{
    client::{
        cache_redis::{self, Cache, CachePool},
        db_mongo,
    },
    util::actix_json_config::json_extractor_config,
};
use std::sync::Arc;
use tracing_actix_web::TracingLogger;

pub async fn start_web_service(
    app_name: &str,
    configuration: Settings,
) -> Result<(), std::io::Error> {
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    let db_client = db_mongo::connect(app_name, &configuration.db)
        .await
        .expect("db client connection failure");

    let cache_pool: CachePool = cache_redis::connect(&configuration.cache)?;
    let cache_client: Cache = cache_redis::Cache::new(cache_pool);

    // Instantiate the application context. This application state will be
    // cloned for each Actix thread but the Arc of the DbContext will be
    // reused in each Actix thread.
    let app_context = web::Data::new(AppContext {
        db: Arc::new(db_client),
        cache: Arc::new(cache_client),
    });

    let governor_conf = GovernorConfigBuilder::default()
        .per_second(configuration.rate_limit.frequency)
        .burst_size(configuration.rate_limit.burst_size)
        .finish()
        .unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_context.clone())
            .app_data(json_extractor_config(4096))
            .wrap(TracingLogger::default())
            .wrap(RequestTracing::new())
            // Enable Governor middleware
            .wrap(Governor::new(&governor_conf))
            .service(web::scope("/api/v1.0").configure(controller::global_router))
            .default_service(web::get().to(not_found))
    })
    .bind(&address)?;

    server
        .workers(configuration.application.workers)
        .run()
        .await?;

    Ok(())
}

pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("the requested resource does not exist")
}
