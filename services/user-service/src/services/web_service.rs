use crate::controllers;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

pub async fn start_web_service() {
    let _ = HttpServer::new(move || {
        App::new()
            .service(web::scope("/api/v1.0").configure(controllers::global_router))
            .default_service(web::get().to(not_found))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await;
}

pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("the requested resource does not exist")
}
