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
#[derive(Serialize)]
struct Health {
    healthy: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

pub async fn health(ctx: web::Data<AppContext>) -> Result<HttpResponse, InternalError> {
    let mut health = HashMap::<&str, Health>::new();
    health.insert("mongodb", mongo_health(&ctx).await);

    let status = match health.values().any(|health| !health.healthy) {
        true => StatusCode::SERVICE_UNAVAILABLE,
        false => StatusCode::OK,
    };

    Ok(HttpResponseBuilder::new(status).json(json!(
        {
            "MongoDB": health["mongodb"],
        }
    )))
}

async fn mongo_health(ctx: &web::Data<AppContext>) -> Health {
    match db_mongo::ping(ctx.db()).await {
        Err(err) => Health {
            healthy: false,
            message: Some(err.to_string()),
        },
        Ok(_) => Health {
            healthy: true,
            message: None,
        },
    }
}
