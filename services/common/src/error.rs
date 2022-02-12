use actix_http::{
    error::{BlockingError, PayloadError},
    header::{InvalidHeaderName, InvalidHeaderValue},
};
use actix_web::{
    error::JsonPayloadError,
    http::StatusCode,
    HttpResponse,
    HttpResponseBuilder,
    ResponseError,
};
use awc::error::SendRequestError;

use crossbeam_channel::SendError;
use derive_more::{Display, Error};
use lazy_static::lazy_static;
use mongodb::{
    bson::{self, document::ValueAccessError},
    error::{ErrorKind, WriteFailure},
};
use parking_lot::RwLock;
use serde_json::json;
use tracing::error;
use url::ParseError;
use uuid::Uuid;

pub type ApiResult = Result<actix_web::HttpResponse, InternalError>;

lazy_static! {
    pub static ref REDACTED_ERRORS: RwLock<bool> = RwLock::new(true);
}

/// An error type used throughout the services code which can be converted into
/// a HTTP error response.
///
/// All possible library or system errors are converted into one of these
/// InternalErrors so our code can have a clean Result<blah, InternalError>
/// signature declaration and avoids excessive use or operation.await.
/// map_err(|err| blah) type call.
///
/// Conversion from a source error to an InternalError is done futher below with
/// a series of From<T> trait implementations.
#[derive(Clone, Debug, Display, Error)]
pub enum InternalError {
    #[display(fmt = "Db error: {}", cause)]
    DbError { cause: String },

    #[display(
        fmt = "Db schema needs to be v{} but it is v{} - try running with UPDATE_SCHEMA_ENABLED \
               set",
        code_version,
        db_version
    )]
    DbSchemaError { code_version: i32, db_version: i32 },

    #[display(
        fmt = "Db is locked for schema updating. Either another instance has locked it and is \
               taking a long time or a previous update crashed and left the lock in place. {}",
        cause
    )]
    DbLockedForUpdate { cause: String },

    #[display(fmt = "The request had no fields to update")]
    DbUpdateEmpty,

    #[display(
        fmt = "The request is not allowed and would cause a duplicate value: {}",
        cause
    )]
    DbDuplicateError { cause: String },

    #[display(fmt = "Failed to execute the task on blocking thread: {}", cause)]
    BlockingTaskExecutionError { cause: String },

    #[display(fmt = "Request format invalid: {}", reason)]
    RequestFormatError { reason: String },

    #[display(fmt = "Failed to make downstream request: {}", cause)]
    SendRequestError { cause: String },

    #[display(fmt = "{} claim invalid", claim)]
    InvalidClaim { claim: String },

    #[display(fmt = "Url could not be parsed: {}", cause)]
    InvalidUrl { cause: String },

    #[display(fmt = "Unable to convert to bson: {}", cause)]
    InvalidBsonError { cause: String },

    #[display(fmt = "Unable to convert to json: {}", cause)]
    InvalidJsonError { cause: String },

    #[display(fmt = "Unable to read bson: {}", cause)]
    BsonAccessError { cause: String },

    #[display(fmt = "Request to {} failed with {}", url, cause)]
    RemoteRequestError { cause: String, url: String },

    #[display(fmt = "User {} not found", user_id)]
    UserNotFound { user_id: Uuid },

    #[display(fmt = "Failed to internally notify: {}", cause)]
    SendNotificationError { cause: String },

    #[display(fmt = "InvalidFormatError: {}", cause)]
    InvalidFormatError { cause: String },
}

impl InternalError {
    fn error_code(&self) -> u16 {
        match *self {
            InternalError::InvalidFormatError { cause: _ } => 400,
            InternalError::InvalidClaim { claim: _ } => 1000,
            InternalError::RemoteRequestError { cause: _, url: _ } => 1005,
            InternalError::RequestFormatError { reason: _ } => 1010,
            InternalError::DbError { cause: _ } => 2001,
            InternalError::DbSchemaError {
                code_version: _,
                db_version: _,
            } => 2002,
            InternalError::DbLockedForUpdate { cause: _ } => 2003,
            InternalError::DbUpdateEmpty => 2004,
            InternalError::DbDuplicateError { cause: _ } => 2005,
            InternalError::InvalidBsonError { cause: _ } => 2006,
            InternalError::InvalidJsonError { cause: _ } => 2105,
            InternalError::InvalidUrl { cause: _ } => 2150,
            InternalError::BsonAccessError { cause: _ } => 2207,
            InternalError::UserNotFound { user_id: _ } => 2509,
            InternalError::SendNotificationError { cause: _ } => 2920,
            InternalError::SendRequestError { cause: _ } => 3000,
            InternalError::BlockingTaskExecutionError { cause: _ } => 3100,
        }
    }

    /// Only 400 (bad request) responses can return an error message field.
    /// It is then controlled via the global redaction flag.
    fn redacted_errors(&self) -> bool {
        if self.status_code() != StatusCode::BAD_REQUEST {
            return true;
        }
        *REDACTED_ERRORS.read()
    }
}

impl ResponseError for InternalError {
    fn status_code(&self) -> StatusCode {
        match *self {
            InternalError::InvalidFormatError { cause: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            InternalError::InvalidClaim { claim: _ } => StatusCode::FORBIDDEN,
            InternalError::RemoteRequestError { cause: _, url: _ } => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            InternalError::DbSchemaError {
                code_version: _,
                db_version: _,
            } => StatusCode::INTERNAL_SERVER_ERROR,
            InternalError::DbLockedForUpdate { cause: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            InternalError::DbError { cause: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            InternalError::DbUpdateEmpty => StatusCode::BAD_REQUEST,
            InternalError::DbDuplicateError { cause: _ } => StatusCode::BAD_REQUEST,
            InternalError::RequestFormatError { reason: _ } => StatusCode::BAD_REQUEST,
            InternalError::InvalidUrl { cause: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            InternalError::InvalidJsonError { cause: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            InternalError::InvalidBsonError { cause: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            InternalError::BsonAccessError { cause: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            InternalError::UserNotFound { user_id: _ } => StatusCode::UNPROCESSABLE_ENTITY,
            InternalError::SendNotificationError { cause: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            InternalError::SendRequestError { cause: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            InternalError::BlockingTaskExecutionError { cause: _ } => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    fn error_response(&self) -> HttpResponse {
        error!("{}", self);

        let body = match self.redacted_errors() {
            true => json!(
            {
                "errorCode": self.error_code()
            }),
            false => json!(
            {
                "errorCode": self.error_code(),
                "message": self.to_string()
            }),
        };

        HttpResponseBuilder::new(self.status_code()).json(body)
    }
}

impl From<BlockingError> for InternalError {
    fn from(err: BlockingError) -> Self {
        InternalError::BlockingTaskExecutionError {
            cause: err.to_string(),
        }
    }
}

impl<T> From<SendError<T>> for InternalError {
    fn from(err: SendError<T>) -> Self {
        InternalError::SendNotificationError {
            cause: err.to_string(),
        }
    }
}

impl From<mongodb::error::Error> for InternalError {
    fn from(error: mongodb::error::Error) -> Self {
        if let ErrorKind::Write(WriteFailure::WriteError(write_error)) = &*error.kind {
            if write_error.code == 11000
            // Duplicate key violation
            {
                return InternalError::DbDuplicateError {
                    cause: error.to_string(),
                };
            }
        }

        InternalError::DbError {
            cause: error.to_string(),
        }
    }
}

impl From<bson::ser::Error> for InternalError {
    fn from(error: bson::ser::Error) -> Self {
        InternalError::InvalidBsonError {
            cause: error.to_string(),
        }
    }
}

impl From<bson::de::Error> for InternalError {
    fn from(error: bson::de::Error) -> Self {
        InternalError::InvalidBsonError {
            cause: error.to_string(),
        }
    }
}

impl From<serde_json::Error> for InternalError {
    fn from(error: serde_json::Error) -> Self {
        InternalError::InvalidJsonError {
            cause: error.to_string(),
        }
    }
}

impl From<InternalError> for std::io::Error {
    fn from(error: InternalError) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::Other, error.to_string())
    }
}

impl From<ValueAccessError> for InternalError {
    fn from(error: ValueAccessError) -> Self {
        InternalError::BsonAccessError {
            cause: error.to_string(),
        }
    }
}

impl From<ParseError> for InternalError {
    fn from(error: ParseError) -> Self {
        InternalError::InvalidUrl {
            cause: error.to_string(),
        }
    }
}

impl From<PayloadError> for InternalError {
    fn from(error: PayloadError) -> Self {
        InternalError::InvalidJsonError {
            cause: error.to_string(),
        }
    }
}

impl From<SendRequestError> for InternalError {
    fn from(error: SendRequestError) -> Self {
        InternalError::SendRequestError {
            cause: error.to_string(),
        }
    }
}

impl From<JsonPayloadError> for InternalError {
    fn from(error: JsonPayloadError) -> Self {
        InternalError::InvalidJsonError {
            cause: error.to_string(),
        }
    }
}

impl From<InvalidHeaderName> for InternalError {
    fn from(error: InvalidHeaderName) -> Self {
        InternalError::SendRequestError {
            cause: error.to_string(),
        }
    }
}

impl From<InvalidHeaderValue> for InternalError {
    fn from(error: InvalidHeaderValue) -> Self {
        InternalError::SendRequestError {
            cause: error.to_string(),
        }
    }
}

impl From<std::fmt::Error> for InternalError {
    fn from(_: std::fmt::Error) -> Self {
        todo!()
    }
}
