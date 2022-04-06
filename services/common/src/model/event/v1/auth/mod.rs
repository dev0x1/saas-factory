use serde::{Deserialize, Serialize};

pub mod prelude {
    pub const SERVICE_AUTH_SUBJECT: &str = "service.auth";

    pub const SERVICE_AUTH_COMMAND_SEND_OTP: &str = "cmd.send.otp";

    pub const SERVICE_AUTH_EVENT_USER_CREATED: &str = "evt.user.created";
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SendOtpMessage {
    pub id: String,
    pub email: String,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct UserCreatedMessage {
    pub id: String,
    pub email: String,
}
