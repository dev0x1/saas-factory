use mongodb::bson::{self, doc, Bson};
use redis::{FromRedisValue, RedisResult, RedisWrite, ToRedisArgs, Value};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::{fmt, fmt::Formatter};
use strum::{Display, EnumString};

pub mod prelude {
    // Collection name
    pub const COLLECTION_USERS: &str = "users";

    // User fields.
    pub const ID: &str = "_id";
    pub const EMAIL: &str = "EMAIL";
    pub const STATUS: &str = "STATUS";
    pub const ROLE: &str = "ROLE";
    pub const CREATED_AT: &str = "CREATED_AT";
    pub const UPDATED_AT: &str = "UPDATED_AT";

    // Cache keys
    pub const CACHE_KEY_PREFIX_USER_ID: &str = "_id";
    pub const CACHE_USER_EXPIRY: usize = 600;
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Display, EnumString)]
#[strum(serialize_all = "UPPERCASE")]
pub enum UserStatus {
    Invited,
    Active,
    Inactive,
}

impl From<UserStatus> for Bson {
    fn from(status: UserStatus) -> Self {
        match status {
            UserStatus::Invited => Bson::String("Invited".to_string()),
            UserStatus::Active => Bson::String("Active".to_string()),
            UserStatus::Inactive => Bson::String("Inactive".to_string()),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Display, EnumString)]
#[strum(serialize_all = "UPPERCASE")]
pub enum UserRole {
    Admin,
    User,
}

impl From<UserRole> for Bson {
    fn from(status: UserRole) -> Self {
        match status {
            UserRole::Admin => Bson::String("Admin".to_string()),
            UserRole::User => Bson::String("User".to_string()),
        }
    }
}

//#[serde_with::serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: Option<bson::Uuid>,
    #[serde(rename = "EMAIL")]
    pub email: Option<String>,
    #[serde(rename = "STATUS")]
    pub status: Option<UserStatus>,
    #[serde(rename = "ROLE")]
    pub role: Option<UserRole>,
    // #[serde_as(as = "Option<bson::DateTime>")]
    #[serde(rename = "CREATED_AT")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    // #[serde_as(as = "Option<bson::DateTime>")]
    #[serde(rename = "UPDATED_AT")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl ToRedisArgs for User {
    fn write_redis_args<W>(&self, output: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        output.write_arg_fmt(serde_json::to_string(self).unwrap());
    }
}

impl FromRedisValue for User {
    fn from_redis_value(value: &Value) -> RedisResult<Self> {
        match *value {
            redis::Value::Data(ref value_slice) => match serde_json::from_slice(value_slice) {
                Err(_) => Err((redis::ErrorKind::TypeError, "Can't serialize value").into()),
                Ok(user) => Ok(user),
            },
            _ => Err((
                redis::ErrorKind::ResponseError,
                "Response type not Profile compatible.",
            )
                .into()),
        }
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
