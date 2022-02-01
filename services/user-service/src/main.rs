use user_service::services;
use user_service::settings;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = settings::get_configuration().expect("Failed to read configuration.");

    // Start Web server
    services::start_web_service(configuration).await
}
