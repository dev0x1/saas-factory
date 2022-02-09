use actix_web::{
    web::{self},
    HttpResponse, Scope,
};
use chrono::Utc;
use common::error::ApiResult;
use tracing::error;
use uuid::Uuid;

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

///
/// Http handler for querying users.
///
#[tracing::instrument(name = "query", skip(user), level = "info")]
pub async fn query(ctx: web::Data<AppContext>, web::Query(user): web::Query<User>) -> ApiResult {
    match user.id {
        Some(_) => get_by_id(ctx, user).await,
        None => get_by_condition(ctx, user).await,
    }
}

async fn get_by_id(ctx: web::Data<AppContext>, user: User) -> ApiResult {
    if let Some(id) = user.id {
        let res = user_repository::find_by_id(&id, ctx.db()).await;
        match res {
            Ok(info) => match info {
                Some(info) => Ok(HttpResponse::Ok()
                    .content_type("application/json")
                    .json(info)),
                None => Ok(HttpResponse::NotFound()
                    .content_type("plain/text")
                    .body("no such id")),
            },
            Err(err) => {
                error!("failed to get by id {}, detail: {}", &id, err);
                Ok(HttpResponse::InternalServerError()
                    .content_type("plain/text")
                    .body(err.to_string()))
            }
        }
    } else {
        Ok(HttpResponse::UnprocessableEntity()
            .content_type("plain/text")
            .body("no `_id` field received"))
    }
}

async fn get_by_condition(ctx: web::Data<AppContext>, user: User) -> ApiResult {
    let res = user_repository::find_all(&user, ctx.db()).await;
    match res {
        Ok(users) => Ok(HttpResponse::Ok()
            .content_type("application/json")
            .json(users)),
        Err(err) => {
            error!("failed to query by condition: {}, detail: {}", user, err);
            Ok(HttpResponse::InternalServerError()
                .content_type("plain/text")
                .body(err.to_string()))
        }
    }
}

///
/// Http handler for creating an user.
///
#[tracing::instrument(name = "create", skip(user), level = "info")]
pub async fn create(ctx: web::Data<AppContext>, user: web::Json<User>) -> ApiResult {
    // verify necessary fields
    if user.email == None {
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("plain/text")
            .body("require fields: `EMAIL`"));
    }

    let now = Some(Utc::now());
    let to_create = User {
        id: Some(Uuid::new_v4()),
        status: Some(UserStatus::Active),
        role: Some(UserRole::User),
        created_at: now,
        updated_at: now,
        ..user.0
    };

    let res = user_repository::insert_one(&to_create, ctx.db()).await;
    match res {
        Ok(user) => Ok(HttpResponse::Ok()
            .content_type("application/json")
            .json(user)),
        Err(err) => {
            error!("failed to create: {}, detail: {}", to_create, err);
            Ok(HttpResponse::InternalServerError()
                .content_type("plain/text")
                .body(err.to_string()))
        }
    }
}

///
/// Http handler for updating an user.
///
#[tracing::instrument(name = "update_by_id", skip(user), level = "info")]
pub async fn update_by_id(ctx: web::Data<AppContext>, user: web::Json<User>) -> ApiResult {
    // verify necessary fields
    if user.id == None {
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("plain/text")
            .body("require fields: `_id`"));
    }

    let res = user_repository::update_by_id(&user, ctx.db()).await;
    match res {
        Ok(user) => Ok(HttpResponse::Ok()
            .content_type("application/json")
            .json(user)),
        Err(err) => {
            error!("failed to update: {}, detail: {}", user, err);
            Ok(HttpResponse::InternalServerError()
                .content_type("plain/text")
                .body(err.to_string()))
        }
    }
}

///
/// Http handler for deleting an user.
///
#[tracing::instrument(name = "delete_by_id", skip(user), level = "info")]
pub async fn delete_by_id(
    ctx: web::Data<AppContext>,
    web::Query(user): web::Query<User>,
) -> ApiResult {
    // verify necessary fields
    if user.id == None {
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("plain/text")
            .body("require fields: `_id`"));
    }

    let id = user.id.unwrap();
    let res = user_repository::delete_one(&id, ctx.db()).await;
    match res {
        Ok(_) => Ok(HttpResponse::NoContent()
            .content_type("plain/text")
            .body("")),
        Err(err) => {
            error!("failed to delete by id {}, detail: {}", id, err);
            Ok(HttpResponse::InternalServerError()
                .content_type("plain/text")
                .body(err.to_string()))
        }
    }
}
