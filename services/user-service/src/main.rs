use user_service::services;
use user_service::settings::get_settings;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let settings = get_settings().expect("Failed to read configuration.");

    // Start Web server
    services::start_web_service(settings).await
}
