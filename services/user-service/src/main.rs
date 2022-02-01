use common::utils::telemetry;
use user_service::services;
use user_service::settings::get_settings;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    // configure tracing subscriber
    let app_name = concat!(env!("CARGO_PKG_NAME"), "-", env!("CARGO_PKG_VERSION")).to_string();
    telemetry::config_telemetry(&app_name);

    let settings = get_settings().expect("Failed to read configuration.");

    // Start Web server
    services::start_web_service(settings).await?;
    // Ensure all spans have been reported
    opentelemetry::global::shutdown_tracer_provider();
    Ok(())
}
