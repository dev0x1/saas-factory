pub fn global_router(cfg: &mut actix_web::web::ServiceConfig) {
    use super::*;

    cfg.service(tenant_controller::router());
    cfg.service(health_controller::router());
}
