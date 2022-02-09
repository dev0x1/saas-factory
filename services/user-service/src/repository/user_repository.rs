use crate::model::domain::user::{prelude::*, User};
use futures::StreamExt;
use mongodb::{
    bson::{doc, Document},
    options::FindOptions,
    Database,
};
use uuid::Uuid;

pub async fn find_by_id(id: &Uuid, db: &Database) -> Result<Option<User>, mongodb::error::Error> {
    let filter = doc! { "_id": id };
    db.collection::<User>(COLLECTION_USERS)
        .find_one(filter, None)
        .await
}

pub async fn find_all(cond: &User, db: &Database) -> Result<Vec<User>, mongodb::error::Error> {
    let find_opts = FindOptions::builder().sort(doc! {"_id": 1}).build();

    let mut doc = Document::new();
    if let Some(id) = cond.id {
        doc.insert("_id", id);
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
    if let Some(status) = &cond.status {
        doc.insert(STATUS, &status);
    }
    if let Some(role) = &cond.role {
        doc.insert(ROLE, &role);
    }
    if let Some(created_at) = &cond.created_at {
        doc.insert(CREATED_AT, &created_at);
    }
    if let Some(updated_at) = &cond.updated_at {
        doc.insert(UPDATED_AT, &updated_at);
    }

    let mut cursor = db
        .collection::<User>(COLLECTION_USERS)
        .find(doc, find_opts)
        .await?;
    let mut ret: Vec<User> = Vec::new();
    while let Some(info) = cursor.next().await {
        match info {
            Ok(info) => ret.push(info),
            Err(err) => return Err(err),
        }
    }
    Ok(ret)
}

pub async fn insert_one(user: &User, db: &Database) -> Result<(), mongodb::error::Error> {
    db.collection::<User>(COLLECTION_USERS)
        .insert_one(user, None)
        .await
        .and_then(|_| Ok(()))
}

pub async fn update_by_id(user: &User, db: &Database) -> Result<User, mongodb::error::Error> {
    let ret = user.clone();

    let id = user.id.unwrap();
    let query = doc! { "_id": id };

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

    db.collection::<User>(COLLECTION_USERS)
        .update_one(
            query,
            doc! {
                "$set": update,
            },
            None,
        )
        .await
        .and_then(|_| Ok(ret))
}

pub async fn delete_one(id: &Uuid, db: &Database) -> Result<(), mongodb::error::Error> {
    db.collection::<User>(COLLECTION_USERS)
        .delete_one(
            doc! {
                "_id" : id
            },
            None,
        )
        .await
        .and_then(|_| Ok(()))
}
