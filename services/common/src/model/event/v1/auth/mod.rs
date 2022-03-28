use actix::Message;
use serde::{Deserialize, Serialize};

use super::EventMetadata;

pub mod prelude {
    pub const SERVICE_AUTH_SUBJECT: &str = "service.auth";

    pub const SERVICE_AUTH_EVENT_USER_CREATED: &str = "user.created";
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct UserCreatedMessage {
    pub id: String,
    pub email: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub struct UserCreated {
    pub meta: EventMetadata,
    pub payload: UserCreatedMessage,
}
