use actix_web::{web, HttpResponse, Scope};

pub fn router() -> Scope {
    web::scope("/health").service(web::resource("").route(web::get().to(get_health)))
}

async fn get_health() -> HttpResponse {
    HttpResponse::Ok().body("service is up")
}
