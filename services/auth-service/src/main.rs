#[actix_web::main]
async fn main() -> std::io::Result<()> {
    auth_service::server().await
}
