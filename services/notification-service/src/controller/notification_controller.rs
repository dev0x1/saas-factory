use actix_web::{
    web::{self},
    HttpResponse,
    Scope,
};

use chrono::Utc;
use common::error::{ApiResult, InternalError};

use crate::{context::AppContext, repository::notification_repository};

pub fn router() -> Scope {
    web::scope("notification").service(
        web::resource("")
            .route(web::get().to(query))
            .route(web::post().to(create)),
    )
}

/// Http handler for querying a resource.
#[tracing::instrument(name = "query", level = "info")]
pub async fn query(ctx: web::Data<AppContext>) -> ApiResult {
    todo!()
}

/// Http handler for creating a resource.
#[tracing::instrument(name = "create", level = "info")]
pub async fn create(ctx: web::Data<AppContext>) -> ApiResult {
    todo!()
}
