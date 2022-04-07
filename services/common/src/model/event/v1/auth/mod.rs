use actix::Message;
use serde::{Deserialize, Serialize};

pub mod prelude {
    pub const SERVICE_AUTH_SUBJECT: &str = "service.auth";

    pub const SERVICE_AUTH_COMMAND_SEND_OTP: &str = "cmd.send.otp";

    pub const SERVICE_AUTH_EVENT_USER_CREATED: &str = "evt.user.created";
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Message)]
#[rtype(result = "Result<(), std::io::Error>")]
pub struct SendOtpMessage {
    pub from: String,
    pub to: String,
    pub sub: String,
    pub body: String,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Message)]
#[rtype(result = "Result<(), std::io::Error>")]
pub struct UserCreatedMessage {
    pub user_id: String,
    pub email: String,
}
