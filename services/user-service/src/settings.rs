use common::{client::db_mongo::MongoClientSettings, util::configuration};
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub db: MongoClientSettings,
    pub log: LogSettings,
    pub rate_limit: RateLimitingSettings,
    pub tracer: Tracer,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub base_url: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub workers: usize,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub max_json_payload_size: usize,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct LogSettings {
    pub level: String,
    pub rust_log: String,
    pub rust_backtrace: String,
    pub redacted_errors: bool,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Tracer {
    pub jaeger: Jaeger,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Jaeger {
    pub url: String,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct RateLimitingSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub burst_size: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub frequency: u64,
}

impl Settings {
    pub fn load() -> Result<Settings, config::ConfigError> {
        configuration::load_configuration::<Settings>()
    }
}
