use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::{fmt, fmt::Formatter};

pub mod prelude {
    // Collection name
    pub const COLLECTION_LOGIN_ATTEMPTS: &str = "login_attempts";

    // LoginAttempt fields.
    pub const EMAIL: &str = "EMAIL";
    pub const OTP_CODE: &str = "OTP_CODE";
    pub const OTP_REQUEST_COUNT: &str = "OTP_REQUESTS_COUNT";
}

//#[serde_with::serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginAttempt {
    #[serde(rename = "EMAIL")]
    pub email: String,
    #[serde(rename = "OTP_CODE")]
    pub otp_code: bson::Uuid,
    #[serde(rename = "OTP_REQUESTS_COUNT")]
    pub otp_requests_count: usize,
}

impl fmt::Display for LoginAttempt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
