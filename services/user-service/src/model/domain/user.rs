use mongodb::bson::Bson;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::fmt;
use std::fmt::Formatter;
use strum::{Display, EnumString};
use uuid::Uuid;

pub mod prelude {
    // Collection name
    pub const COLLECTION_USERS: &str = "users";

    // User fields.
    pub const ID: &str = "_id";
    pub const EMAIL: &str = "EMAIL";
    pub const FIRST_NAME: &str = "FIRST_NAME";
    pub const LAST_NAME: &str = "LAST_NAME";
    pub const PHONE: &str = "PHONE";
    pub const STATUS: &str = "STATUS";
    pub const ROLE: &str = "ROLE";
    pub const CREATED_AT: &str = "CREATED_AT";
    pub const UPDATED_AT: &str = "UPDATED_AT";
}

#[derive(
    Copy,
    Clone,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Debug,
    Display,
    EnumString
)]
#[strum(serialize_all = "UPPERCASE")]
pub enum UserStatus {
    Active,
    Inactive,
}

impl From<UserStatus> for Bson {
    fn from(status: UserStatus) -> Self {
        match status {
            UserStatus::Active => Bson::String("Active".to_string()),
            UserStatus::Inactive => Bson::String("Inactive".to_string()),
        }
    }
}

#[derive(
    Copy,
    Clone,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Debug,
    Display,
    EnumString
)]
#[strum(serialize_all = "UPPERCASE")]
pub enum UserRole {
    Admin,
    User,
}

impl From<UserRole> for Bson {
    fn from(status: UserRole) -> Self {
        match status {
            UserRole::Admin => Bson::String("Active".to_string()),
            UserRole::User => Bson::String("Inactive".to_string()),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: Option<Uuid>,
    #[serde(rename = "EMAIL")]
    pub email: Option<String>,
    #[serde(rename = "FIRST_NAME")]
    pub first_name: Option<String>,
    #[serde(rename = "LAST_NAME")]
    pub last_name: Option<String>,
    #[serde(rename = "PHONE")]
    pub phone_number: Option<String>,
    #[serde(rename = "STATUS")]
    pub status: Option<UserStatus>,
    #[serde(rename = "ROLE")]
    pub role: Option<UserRole>,
    #[serde(rename = "CREATED_AT")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "UPDATED_AT")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
