#[actix_web::main]
async fn main() -> std::io::Result<()> {
    notification_service::server().await
}
