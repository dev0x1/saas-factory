use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Clone, Validate)]
pub struct RegisterUser {
    #[validate(required, email(message = "email is not valid"))]
    pub email: Option<String>,
}
