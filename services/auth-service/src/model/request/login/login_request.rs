use bson::Uuid;
use serde::Deserialize;
use validator::Validate;

use crate::model::domain::user::UserRole;

#[derive(Debug, Deserialize, Clone, Validate)]
pub struct Invite {
    #[validate(required, email(message = "email is not valid"))]
    pub email: Option<String>,
    #[validate(required)]
    #[serde(rename = "ROLE")]
    pub role: Option<UserRole>,
}

#[derive(Debug, Deserialize, Clone, Validate)]
pub struct InviteConfirmation {
    #[validate(required, email(message = "email is not valid"))]
    pub email: Option<String>,

    #[validate(required)]
    #[serde(rename = "otpCode")]
    pub otp_code: Option<Uuid>,
}

#[derive(Debug, Deserialize, Clone, Validate)]
pub struct Identify {
    #[validate(required, email(message = "email is not valid"))]
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Validate)]
pub struct Verify {
    #[validate(required)]
    pub id: Option<Uuid>,
}
