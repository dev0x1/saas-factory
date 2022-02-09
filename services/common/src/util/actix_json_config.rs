use actix_web::web::JsonConfig;

use crate::error::InternalError;

///
/// Return JSON parse details as an error to the client.
///
pub fn json_extractor_config(max_payload_size: usize) -> JsonConfig {
    JsonConfig::default()
        .limit(max_payload_size)
        .error_handler(|err, _req| {
            InternalError::RequestFormatError {
                reason: err.to_string(),
            }
            .into()
        })
}
