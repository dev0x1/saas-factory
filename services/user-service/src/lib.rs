mod context;
mod controller;
mod model;
mod repository;
mod service;
mod settings;

use common::{error::REDACTED_ERRORS, util::telemetry};
use settings::Settings;

pub async fn server() -> Result<(), std::io::Error> {
    // configure tracing subscriber
    let app_name = concat!(env!("CARGO_PKG_NAME"), "-", env!("CARGO_PKG_VERSION")).to_string();

    let settings = Settings::load().expect("Failed to read configuration.");
    *REDACTED_ERRORS.write() = settings.log.redacted_errors;

    telemetry::config_telemetry(&app_name, &settings.tracer.jaeger.url);

    // Start Web server
    service::start_web_service(&app_name, settings).await?;
    // Ensure all spans have been reported
    opentelemetry::global::shutdown_tracer_provider();
    Ok(())
}
