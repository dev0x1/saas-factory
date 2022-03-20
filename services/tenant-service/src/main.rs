#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tenant_service::server().await
}
