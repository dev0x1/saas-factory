pub fn global_router(cfg: &mut actix_web::web::ServiceConfig) {
    use super::*;

    cfg.service(notification_controller::router());
    cfg.service(health_controller::router());
}
