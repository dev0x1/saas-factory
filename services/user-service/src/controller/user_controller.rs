use actix_web::{
    web::{self},
    HttpResponse,
    Scope,
};
use bson::Uuid;
use chrono::Utc;
use common::error::{ApiResult, InternalError};

use crate::{
    context::AppContext,
    model::domain::user::{User, UserRole, UserStatus},
    repository::user_repository,
};

pub fn router() -> Scope {
    web::scope("/user").service(
        web::resource("")
            .route(web::get().to(query))
            .route(web::post().to(create))
            .route(web::put().to(update_by_id))
            .route(web::delete().to(delete_by_id)),
    )
}

/// Http handler for querying users.
#[tracing::instrument(name = "query", skip(user), level = "info")]
pub async fn query(ctx: web::Data<AppContext>, web::Query(user): web::Query<User>) -> ApiResult {
    match user.id {
        Some(id) => get_by_id(ctx, &id).await,
        None => get_by_condition(ctx, user).await,
    }
}

async fn get_by_id(ctx: web::Data<AppContext>, id: &Uuid) -> ApiResult {
    let user = user_repository::find_by_id(id, ctx.db()).await?;

    match user {
        Some(user) => Ok(HttpResponse::Ok()
            .content_type("application/json")
            .json(user)),
        None => Err(InternalError::UserNotFound {
            user_id: id.to_uuid_0_8(),
        }),
    }
}

async fn get_by_condition(ctx: web::Data<AppContext>, user: User) -> ApiResult {
    let users = user_repository::find_all_with_query(&user, ctx.db()).await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(users))
}

/// Http handler for creating an user.
#[tracing::instrument(name = "create", skip(user), level = "info")]
pub async fn create(ctx: web::Data<AppContext>, user: web::Json<User>) -> ApiResult {
    // verify necessary fields
    if user.email.is_none() {
        return Err(InternalError::RequestFormatError {
            reason: "require fields: `EMAIL`".to_string(),
        });
    }

    let now = Some(Utc::now());
    let to_create = User {
        id: Some(bson::Uuid::new()),
        status: Some(UserStatus::Active),
        role: Some(UserRole::User),
        created_at: now,
        updated_at: now,
        ..user.0
    };

    let user = user_repository::insert_one(&to_create, ctx.db()).await?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(user))
}

/// Http handler for updating an user.
#[tracing::instrument(name = "update_by_id", skip(user), level = "info")]
pub async fn update_by_id(ctx: web::Data<AppContext>, user: web::Json<User>) -> ApiResult {
    // verify necessary fields
    if user.id.is_none() {
        return Err(InternalError::RequestFormatError {
            reason: "require fields: `_id`".to_string(),
        });
    }

    let _: u64 = user_repository::update_by_id(&user, ctx.db()).await?;

    Ok(HttpResponse::Ok().finish())
}

/// Http handler for deleting an user.
#[tracing::instrument(name = "delete_by_id", skip(user), level = "info")]
pub async fn delete_by_id(
    ctx: web::Data<AppContext>,
    web::Query(user): web::Query<User>,
) -> ApiResult {
    let id = user.id.ok_or(InternalError::RequestFormatError {
        reason: "require fields: `_id`".to_string(),
    })?;

    let _: u64 = user_repository::delete_one(&id, ctx.db()).await?;
    Ok(HttpResponse::Ok().finish())
}
