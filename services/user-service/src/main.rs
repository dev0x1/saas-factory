use common::utils::telemetry;
use user_service::services;
use user_service::settings::Settings;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    // configure tracing subscriber
    let app_name = concat!(env!("CARGO_PKG_NAME"), "-", env!("CARGO_PKG_VERSION")).to_string();

    let settings = Settings::load().expect("Failed to read configuration.");
    telemetry::config_telemetry(&app_name, &settings.tracer.jaeger.url);

    // Start Web server
    services::start_web_service(settings).await?;
    // Ensure all spans have been reported
    opentelemetry::global::shutdown_tracer_provider();
    Ok(())
}
