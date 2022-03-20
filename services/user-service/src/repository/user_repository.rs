use crate::model::domain::user::{prelude::*, User};
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

pub async fn find_by_id(id: &Uuid, db: &Database) -> Result<Option<User>, InternalError> {
    let filter = doc! { ID: id };
    let user = db
        .collection::<User>(COLLECTION_USERS)
        .find_one(filter, None)
        .await?;
    Ok(user)
}

pub async fn find_by_email(email: &str, db: &Database) -> Result<Option<User>, InternalError> {
    let filter = doc! { EMAIL: email };
    let user = db
        .collection::<User>(COLLECTION_USERS)
        .find_one(filter, None)
        .await?;
    Ok(user)
}

pub async fn find_all(db: &Database) -> Result<Vec<User>, InternalError> {
    let cursor = db
        .collection::<User>(COLLECTION_USERS)
        .find(doc! {}, None)
        .await?;
    Ok(cursor.try_collect().await?)
}

pub async fn find_all_paginated(
    page: u64,
    page_size: u64,
    db: &Database,
) -> Result<PageResponse<User>, InternalError> {
    let total = db
        .collection::<User>(COLLECTION_USERS)
        .count_documents(doc! {}, None)
        .await?;

    let mut options = FindOptions::default();
    options.skip = Some((page - 1) * page_size);
    options.limit = Some(i64::try_from(page_size)?);

    let cursor = db
        .collection::<User>(COLLECTION_USERS)
        .find(doc! {}, options)
        .await?;
    let users: Vec<User> = cursor.try_collect().await?;

    let total_pages = (total as f64 / page_size as f64).ceil() as u32;

    let page_info = Pagination {
        number_of_elements: users.len(),
        page: page as u32,
        page_size: page_size as u32,
        total_pages,
        total_elements: total as usize,
    };

    Ok(PageResponse {
        data: users,
        page_info,
    })
}

pub async fn find_all_with_query(cond: &User, db: &Database) -> Result<Vec<User>, InternalError> {
    let find_opts = FindOptions::builder().sort(doc! {ID: 1}).build();

    let mut doc = Document::new();
    if let Some(id) = cond.id {
        doc.insert(ID, id);
    }
    if let Some(email) = &cond.email {
        doc.insert(EMAIL, &email);
    }
    if let Some(first_name) = &cond.first_name {
        doc.insert(FIRST_NAME, &first_name);
    }
    if let Some(last_name) = &cond.last_name {
        doc.insert(LAST_NAME, &last_name);
    }
    if let Some(phone_number) = &cond.phone_number {
        doc.insert(PHONE, &phone_number);
    }
    if let Some(status) = cond.status {
        doc.insert(STATUS, status);
    }
    if let Some(role) = cond.role {
        doc.insert(ROLE, role);
    }
    if let Some(created_at) = cond.created_at {
        doc.insert(CREATED_AT, created_at);
    }
    if let Some(updated_at) = cond.updated_at {
        doc.insert(UPDATED_AT, updated_at);
    }

    let cursor = db
        .collection::<User>(COLLECTION_USERS)
        .find(doc, find_opts)
        .await?;
    Ok(cursor.try_collect().await?)
}

pub async fn find_all_paginated_with_query(
    cond: &User,
    page: u64,
    page_size: u64,
    db: &Database,
) -> Result<PageResponse<User>, InternalError> {
    let find_opts = FindOptions::builder().sort(doc! {ID: 1}).build();

    let mut doc = Document::new();
    if let Some(id) = cond.id {
        doc.insert(ID, id);
    }
    if let Some(email) = &cond.email {
        doc.insert(EMAIL, &email);
    }
    if let Some(first_name) = &cond.first_name {
        doc.insert(FIRST_NAME, &first_name);
    }
    if let Some(last_name) = &cond.last_name {
        doc.insert(LAST_NAME, &last_name);
    }
    if let Some(phone_number) = &cond.phone_number {
        doc.insert(PHONE, &phone_number);
    }
    if let Some(status) = cond.status {
        doc.insert(STATUS, status);
    }
    if let Some(role) = cond.role {
        doc.insert(ROLE, role);
    }
    if let Some(created_at) = cond.created_at {
        doc.insert(CREATED_AT, created_at);
    }
    if let Some(updated_at) = cond.updated_at {
        doc.insert(UPDATED_AT, updated_at);
    }

    let cursor = db
        .collection::<User>(COLLECTION_USERS)
        .find(doc, find_opts)
        .await?;
    let users: Vec<User> = cursor.try_collect().await?;
    let total = users.len();

    let total_pages = (total as f64 / page_size as f64).ceil() as u32;

    let page_info = Pagination {
        number_of_elements: total,
        page: page as u32,
        page_size: page_size as u32,
        total_pages,
        total_elements: total as usize,
    };

    Ok(PageResponse {
        data: users,
        page_info,
    })
}

pub async fn insert_one(user: &User, db: &Database) -> Result<User, InternalError> {
    let mut ret = user.clone();

    let res = db
        .collection::<User>(COLLECTION_USERS)
        .insert_one(user, None)
        .await?;

    Ok(from_bson(res.inserted_id).map(|id: Uuid| {
        ret.id = Some(id);
        ret
    })?)
}

pub async fn update_by_id(user: &User, db: &Database) -> Result<u64, InternalError> {
    let id = user.id.ok_or(InternalError::RequestFormatError {
        reason: "require fields: `_id`".to_string(),
    })?;

    let query = doc! { ID: id };

    let mut update = Document::new();
    if let Some(email) = &user.email {
        update.insert(EMAIL, &email);
    }
    if let Some(first_name) = &user.first_name {
        update.insert(FIRST_NAME, &first_name);
    }
    if let Some(last_name) = &user.last_name {
        update.insert(LAST_NAME, &last_name);
    }
    if let Some(phone_number) = &user.phone_number {
        update.insert(PHONE, &phone_number);
    }
    if let Some(status) = &user.status {
        update.insert(STATUS, &status);
    }
    if let Some(role) = &user.role {
        update.insert(ROLE, &role);
    }
    if let Some(created_at) = &user.created_at {
        update.insert(CREATED_AT, &created_at);
    }
    if let Some(updated_at) = &user.updated_at {
        update.insert(UPDATED_AT, &updated_at);
    }

    let res = db
        .collection::<User>(COLLECTION_USERS)
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
        .collection::<User>(COLLECTION_USERS)
        .delete_one(
            doc! {
                ID : id
            },
            None,
        )
        .await?;
    Ok(res.deleted_count)
}
