pub fn global_router(cfg: &mut actix_web::web::ServiceConfig) {
    use super::*;

    login_controller::router(cfg);
    cfg.service(user_controller::router());
    cfg.service(health_controller::router());
}
