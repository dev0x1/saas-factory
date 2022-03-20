use bson::Document;
use mongodb::bson::{self, doc, Bson};
use redis::{FromRedisValue, RedisResult, RedisWrite, ToRedisArgs, Value};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::{fmt, fmt::Formatter};
use strum::{Display, EnumString};

use self::prelude::*;

pub mod prelude {
    // Collection name
    pub const COLLECTION_TENANTS: &str = "tenants";

    // Tenant fields.
    pub const ID: &str = "_id";
    pub const COMPANY_NAME: &str = "COMPANY_NAME";
    pub const ACCOUNT_NAME: &str = "ACCOUNT_NAME";
    pub const OWNER_NAME: &str = "OWNER_NAME";
    pub const EMAIL: &str = "EMAIL";
    pub const PHONE: &str = "PHONE";
    pub const STATUS: &str = "STATUS";
    pub const TIER: &str = "TIER";
    pub const CREATED_AT: &str = "CREATED_AT";
    pub const UPDATED_AT: &str = "UPDATED_AT";

    // Cache keys
    pub const CACHE_KEY_PREFIX_TENANT_ID: &str = "_id";
    pub const CACHE_TENANT_EXPIRY: usize = 600;
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Display, EnumString)]
#[strum(serialize_all = "UPPERCASE")]
pub enum TenantStatus {
    Active,
    Inactive,
}

impl From<TenantStatus> for Bson {
    fn from(status: TenantStatus) -> Self {
        match status {
            TenantStatus::Active => Bson::String("ACTIVE".to_string()),
            TenantStatus::Inactive => Bson::String("INACTIVE".to_string()),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Display, EnumString)]
#[strum(serialize_all = "UPPERCASE")]
pub enum TenantTier {
    Free,
    Standard,
    Premium,
}

impl From<TenantTier> for Bson {
    fn from(status: TenantTier) -> Self {
        match status {
            TenantTier::Free => Bson::String("FREE".to_string()),
            TenantTier::Standard => Bson::String("STANDARD".to_string()),
            TenantTier::Premium => Bson::String("PREMIUM".to_string()),
        }
    }
}

//#[serde_with::serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tenant {
    #[serde(rename = "_id")]
    pub id: Option<bson::Uuid>,
    #[serde(rename = "COMPANY_NAME")]
    pub company_name: Option<String>,
    #[serde(rename = "ACCOUNT_NAME")]
    pub account_name: Option<String>,
    #[serde(rename = "OWNER_NAME")]
    pub owner_name: Option<String>,
    #[serde(rename = "EMAIL")]
    pub email: Option<String>,
    #[serde(rename = "PHONE")]
    pub phone_number: Option<String>,
    #[serde(rename = "STATUS")]
    pub status: Option<TenantStatus>,
    #[serde(rename = "TIER")]
    pub tier: Option<TenantTier>,
    // #[serde_as(as = "Option<bson::DateTime>")]
    #[serde(rename = "CREATED_AT")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    // #[serde_as(as = "Option<bson::DateTime>")]
    #[serde(rename = "UPDATED_AT")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<Tenant> for Document {
    fn from(tenant: Tenant) -> Self {
        let mut doc = Document::new();
        if let Some(id) = tenant.id {
            doc.insert(ID, id);
        }
        if let Some(first_name) = &tenant.company_name {
            doc.insert(COMPANY_NAME, &first_name);
        }
        if let Some(last_name) = &tenant.account_name {
            doc.insert(ACCOUNT_NAME, &last_name);
        }
        if let Some(last_name) = &tenant.owner_name {
            doc.insert(OWNER_NAME, &last_name);
        }
        if let Some(email) = &tenant.email {
            doc.insert(EMAIL, &email);
        }
        if let Some(phone_number) = &tenant.phone_number {
            doc.insert(PHONE, &phone_number);
        }
        if let Some(status) = tenant.status {
            doc.insert(STATUS, status);
        }
        if let Some(tier) = tenant.tier {
            doc.insert(TIER, tier);
        }
        if let Some(created_at) = tenant.created_at {
            doc.insert(CREATED_AT, created_at);
        }
        if let Some(updated_at) = tenant.updated_at {
            doc.insert(UPDATED_AT, updated_at);
        }
        doc
    }
}

impl From<&Tenant> for Document {
    fn from(tenant: &Tenant) -> Self {
        let mut doc = Document::new();
        if let Some(id) = tenant.id {
            doc.insert(ID, id);
        }
        if let Some(first_name) = &tenant.company_name {
            doc.insert(COMPANY_NAME, &first_name);
        }
        if let Some(last_name) = &tenant.account_name {
            doc.insert(ACCOUNT_NAME, &last_name);
        }
        if let Some(last_name) = &tenant.owner_name {
            doc.insert(OWNER_NAME, &last_name);
        }
        if let Some(email) = &tenant.email {
            doc.insert(EMAIL, &email);
        }
        if let Some(phone_number) = &tenant.phone_number {
            doc.insert(PHONE, &phone_number);
        }
        if let Some(status) = tenant.status {
            doc.insert(STATUS, status);
        }
        if let Some(tier) = tenant.tier {
            doc.insert(TIER, tier);
        }
        if let Some(created_at) = tenant.created_at {
            doc.insert(CREATED_AT, created_at);
        }
        if let Some(updated_at) = tenant.updated_at {
            doc.insert(UPDATED_AT, updated_at);
        }
        doc
    }
}

impl ToRedisArgs for Tenant {
    fn write_redis_args<W>(&self, output: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        output.write_arg_fmt(serde_json::to_string(self).unwrap());
    }
}

impl FromRedisValue for Tenant {
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

impl fmt::Display for Tenant {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
