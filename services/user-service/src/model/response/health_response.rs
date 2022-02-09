use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}
