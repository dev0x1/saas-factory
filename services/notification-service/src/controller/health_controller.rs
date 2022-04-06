use std::collections::HashMap;

use actix_http::StatusCode;
use actix_web::{web, HttpResponse, HttpResponseBuilder, Scope};
use common::{client::db_mongo, error::InternalError};
use serde::Serialize;
use serde_json::json;

use crate::context::AppContext;

pub fn router() -> Scope {
    web::scope("/health").service(web::resource("").route(web::get().to(health)))
}

pub async fn health(ctx: web::Data<AppContext>) -> Result<HttpResponse, InternalError> {
    Ok(HttpResponse::Ok().body("healthy"))
}
