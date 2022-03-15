use crate::model::domain::login::{
    prelude::{COLLECTION_LOGIN_ATTEMPTS, EMAIL, OTP_CODE},
    LoginAttempt,
};

use bson::Uuid;
use common::error::InternalError;
use mongodb::{bson::doc, Database};

pub async fn find_by_email(
    email: &str,
    db: &Database,
) -> Result<Option<LoginAttempt>, InternalError> {
    let filter = doc! { EMAIL: email };
    let login_attempt = db
        .collection::<LoginAttempt>(COLLECTION_LOGIN_ATTEMPTS)
        .find_one(filter, None)
        .await?;
    Ok(login_attempt)
}

pub async fn find_by_otp(
    otp_code: &Uuid,
    db: &Database,
) -> Result<Option<LoginAttempt>, InternalError> {
    let filter = doc! { OTP_CODE: otp_code };
    let login_attempt = db
        .collection::<LoginAttempt>(COLLECTION_LOGIN_ATTEMPTS)
        .find_one(filter, None)
        .await?;
    Ok(login_attempt)
}

pub async fn insert_one(login_attempt: &LoginAttempt, db: &Database) -> Result<(), InternalError> {
    let res = db
        .collection::<LoginAttempt>(COLLECTION_LOGIN_ATTEMPTS)
        .insert_one(login_attempt, None)
        .await?;

    Ok(())
}
