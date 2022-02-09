#[actix_web::main]
async fn main() -> std::io::Result<()> {
    user_service::server().await
}
