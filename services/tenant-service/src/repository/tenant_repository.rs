use crate::model::domain::tenant::{prelude::*, Tenant};
use bson::{from_bson, Uuid};
use common::{
    error::InternalError,
    model::{domain::pagination::Pagination, response::page_response::PageResponse},
};
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, Document},
    options::FindOptions,
    Database,
};

pub async fn find_by_id(id: &Uuid, db: &Database) -> Result<Option<Tenant>, InternalError> {
    let filter = doc! { ID: id };
    let tenant = db
        .collection::<Tenant>(COLLECTION_TENANTS)
        .find_one(filter, None)
        .await?;
    Ok(tenant)
}

pub async fn find_by_email(email: &str, db: &Database) -> Result<Option<Tenant>, InternalError> {
    let filter = doc! { EMAIL: email };
    let tenant = db
        .collection::<Tenant>(COLLECTION_TENANTS)
        .find_one(filter, None)
        .await?;
    Ok(tenant)
}

pub async fn find_all(db: &Database) -> Result<Vec<Tenant>, InternalError> {
    let cursor = db
        .collection::<Tenant>(COLLECTION_TENANTS)
        .find(doc! {}, None)
        .await?;
    Ok(cursor.try_collect().await?)
}

pub async fn find_all_paginated(
    page: u64,
    page_size: u64,
    db: &Database,
) -> Result<PageResponse<Tenant>, InternalError> {
    let total = db
        .collection::<Tenant>(COLLECTION_TENANTS)
        .count_documents(doc! {}, None)
        .await?;

    let mut options = FindOptions::default();
    options.skip = Some((page - 1) * page_size);
    options.limit = Some(i64::try_from(page_size)?);

    let cursor = db
        .collection::<Tenant>(COLLECTION_TENANTS)
        .find(doc! {}, options)
        .await?;
    let tenants: Vec<Tenant> = cursor.try_collect().await?;

    let total_pages = (total as f64 / page_size as f64).ceil() as u32;

    let page_info = Pagination {
        number_of_elements: tenants.len(),
        page: page as u32,
        page_size: page_size as u32,
        total_pages,
        total_elements: total as usize,
    };

    Ok(PageResponse {
        data: tenants,
        page_info,
    })
}

pub async fn find_all_with_query(
    cond: &Tenant,
    db: &Database,
) -> Result<Vec<Tenant>, InternalError> {
    let find_opts = FindOptions::builder().sort(doc! {ID: 1}).build();

    let doc: Document = cond.into();

    let cursor = db
        .collection::<Tenant>(COLLECTION_TENANTS)
        .find(doc, find_opts)
        .await?;
    Ok(cursor.try_collect().await?)
}

pub async fn find_all_paginated_with_query(
    cond: &Tenant,
    page: u64,
    page_size: u64,
    db: &Database,
) -> Result<PageResponse<Tenant>, InternalError> {
    let find_opts = FindOptions::builder().sort(doc! {ID: 1}).build();

    let doc: Document = cond.into();

    let cursor = db
        .collection::<Tenant>(COLLECTION_TENANTS)
        .find(doc, find_opts)
        .await?;
    let tenants: Vec<Tenant> = cursor.try_collect().await?;
    let total = tenants.len();

    let total_pages = (total as f64 / page_size as f64).ceil() as u32;

    let page_info = Pagination {
        number_of_elements: total,
        page: page as u32,
        page_size: page_size as u32,
        total_pages,
        total_elements: total as usize,
    };

    Ok(PageResponse {
        data: tenants,
        page_info,
    })
}

pub async fn insert_one(tenant: &Tenant, db: &Database) -> Result<Tenant, InternalError> {
    let mut ret = tenant.clone();

    let res = db
        .collection::<Tenant>(COLLECTION_TENANTS)
        .insert_one(tenant, None)
        .await?;

    Ok(from_bson(res.inserted_id).map(|id: Uuid| {
        ret.id = Some(id);
        ret
    })?)
}

pub async fn update_by_id(tenant: &Tenant, db: &Database) -> Result<u64, InternalError> {
    let id = tenant.id.ok_or(InternalError::RequestFormatError {
        reason: "require fields: `_id`".to_string(),
    })?;

    let query = doc! { ID: id };
    let update: Document = tenant.into();

    let res = db
        .collection::<Tenant>(COLLECTION_TENANTS)
        .update_one(
            query,
            doc! {
                "$set": update,
            },
            None,
        )
        .await?;
    Ok(res.modified_count)
}

pub async fn delete_one(id: &Uuid, db: &Database) -> Result<u64, InternalError> {
    let res = db
        .collection::<Tenant>(COLLECTION_TENANTS)
        .delete_one(
            doc! {
                ID : id
            },
            None,
        )
        .await?;
    Ok(res.deleted_count)
}
