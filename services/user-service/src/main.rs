use user_service::services;

#[actix_web::main]
async fn main() {
    // Start Web server
    services::start_web_service().await;
}
