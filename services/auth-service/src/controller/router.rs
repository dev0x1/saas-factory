pub fn global_router(cfg: &mut actix_web::web::ServiceConfig) {
    use super::*;

    auth_controller::router(cfg);
    cfg.service(health_controller::router());
}
