use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use std::error::Error;

pub type ApiResult = Result<actix_web::HttpResponse, ApiError>;

pub enum ApiError {
    ValidationError(String),
    NotFound(String),
    UnexpectedError(String),
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiStatusResponse {
    pub code: u16,
    pub message: String,
}

impl ApiStatusResponse {
    pub fn new(status_code: StatusCode, message: &str) -> Self {
        Self {
            code: status_code.as_u16(),
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Debug for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}\n", self)?;
        let mut current = self.source();
        while let Some(cause) = current {
            writeln!(f, "Caused by:\n\t{}", cause)?;
            current = cause.source();
        }
        Ok(())
    }
}

impl std::error::Error for ApiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ApiError::ValidationError(_) | ApiError::NotFound(_) | ApiError::UnexpectedError(_) => {
                None
            }
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::ValidationError(e) => {
                HttpResponse::BadRequest().json(ApiStatusResponse::new(StatusCode::BAD_REQUEST, e))
            }
            ApiError::NotFound(e) => {
                HttpResponse::NotFound().json(ApiStatusResponse::new(StatusCode::NOT_FOUND, e))
            }
            _ => HttpResponse::InternalServerError().json(ApiStatusResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                StatusCode::INTERNAL_SERVER_ERROR
                    .canonical_reason()
                    .expect("Failed to get the canonical reason of the StatusCode"),
            )),
        }
    }
}
