use actix_web::{
    web::{self},
    HttpResponse,
    Scope,
};
use bson::Uuid;
use chrono::Utc;
use common::{
    error::{ApiResult, InternalError},
    model::request::page_request::PageRequest,
};
use validator::Validate;

use crate::{
    context::AppContext,
    model::domain::tenant::{
        prelude::{CACHE_KEY_PREFIX_TENANT_ID, CACHE_TENANT_EXPIRY},
        Tenant,
        TenantStatus,
        TenantTier,
    },
    repository::tenant_repository,
};

pub fn router() -> Scope {
    web::scope("/tenant").service(
        web::resource("")
            //.route(web::get().to(query))
            .route(web::get().to(query_paginated))
            .route(web::post().to(create))
            .route(web::put().to(update_by_id))
            .route(web::delete().to(delete_by_id)),
    )
}

/// Http handler for querying tenants.
#[tracing::instrument(name = "query", skip(tenant), level = "info")]
pub async fn query(
    ctx: web::Data<AppContext>,
    web::Query(tenant): web::Query<Tenant>,
) -> ApiResult {
    match tenant.id {
        Some(id) => get_by_id(ctx, &id).await,
        None => get_by_condition(ctx, tenant).await,
    }
}

/// Http handler for querying tenants with pagination.
#[tracing::instrument(name = "query_paginated", skip(tenant, page_request), level = "info")]
pub async fn query_paginated(
    ctx: web::Data<AppContext>,
    web::Query(tenant): web::Query<Tenant>,
    web::Query(page_request): web::Query<PageRequest>,
) -> ApiResult {
    match tenant.id {
        Some(id) => get_by_id(ctx, &id).await,
        None => get_paginated_by_condition(ctx, tenant, page_request).await,
    }
}

async fn get_by_id(ctx: web::Data<AppContext>, id: &Uuid) -> ApiResult {
    let cache_key_tenant_id = format!("{CACHE_KEY_PREFIX_TENANT_ID}_{id}");

    match ctx.cache().get::<Tenant>(&cache_key_tenant_id).await? {
        Some(tenant) => Ok(HttpResponse::Ok()
            .content_type("application/json")
            .json(tenant)),
        None => match tenant_repository::find_by_id(id, ctx.db()).await? {
            Some(tenant) => {
                ctx.cache()
                    .set::<Tenant>(&cache_key_tenant_id, tenant.clone(), CACHE_TENANT_EXPIRY)
                    .await?;
                Ok(HttpResponse::Ok()
                    .content_type("application/json")
                    .json(tenant))
            }
            None => Err(InternalError::TenantNotFound {
                tenant_id: id.to_uuid_0_8(),
            }),
        },
    }
}

async fn get_by_condition(ctx: web::Data<AppContext>, tenant: Tenant) -> ApiResult {
    let tenants = tenant_repository::find_all_with_query(&tenant, ctx.db()).await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(tenants))
}

async fn get_paginated_by_condition(
    ctx: web::Data<AppContext>,
    tenant: Tenant,
    page_request: PageRequest,
) -> ApiResult {
    let tenants = tenant_repository::find_all_paginated_with_query(
        &tenant,
        page_request.page,
        page_request.page_size,
        ctx.db(),
    )
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(tenants))
}

/// Http handler for creating an tenant.
#[tracing::instrument(name = "create", skip(tenant), level = "info")]
pub async fn create(ctx: web::Data<AppContext>, tenant: web::Json<Tenant>) -> ApiResult {
    // verify necessary fields
    if tenant.email.is_none() {
        return Err(InternalError::RequestFormatError {
            reason: "require fields: `EMAIL`".to_string(),
        });
    }

    let now = Some(Utc::now());
    let to_create = Tenant {
        id: Some(bson::Uuid::new()),
        status: Some(TenantStatus::Active),
        created_at: now,
        updated_at: now,
        ..tenant.0
    };

    let tenant = tenant_repository::insert_one(&to_create, ctx.db()).await?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(tenant))
}

/// Http handler for updating an tenant.
#[tracing::instrument(name = "update_by_id", skip(tenant), level = "info")]
pub async fn update_by_id(ctx: web::Data<AppContext>, tenant: web::Json<Tenant>) -> ApiResult {
    // verify necessary fields
    if tenant.id.is_none() {
        return Err(InternalError::RequestFormatError {
            reason: "require fields: `_id`".to_string(),
        });
    }

    let _: u64 = tenant_repository::update_by_id(&tenant, ctx.db()).await?;

    Ok(HttpResponse::Ok().finish())
}

/// Http handler for deleting an tenant.
#[tracing::instrument(name = "delete_by_id", skip(tenant), level = "info")]
pub async fn delete_by_id(
    ctx: web::Data<AppContext>,
    web::Query(tenant): web::Query<Tenant>,
) -> ApiResult {
    let id = tenant.id.ok_or(InternalError::RequestFormatError {
        reason: "require fields: `_id`".to_string(),
    })?;

    let _: u64 = tenant_repository::delete_one(&id, ctx.db()).await?;
    Ok(HttpResponse::Ok().finish())
}
