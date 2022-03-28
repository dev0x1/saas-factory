use common::{
    client::{
        cache_redis::RedisClientSettings,
        db_mongo::MongoClientSettings,
        sm_vault::{VaultClientConfig, VaultKvPath},
    },
    util::configuration,
};
use nats_actor::NatsClientSettings;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub vault: VaultClientConfig,
    pub db: MongoClientSettings,
    pub db_secrets_path: VaultKvPath,
    pub cache: RedisClientSettings,
    pub cache_secrets_path: VaultKvPath,
    pub nats: NatsClientSettings,
    pub log: LogSettings,
    pub tracer: Tracer,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct ApplicationSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub base_url: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub workers: usize,
    pub max_json_payload_size: usize,
    pub nats_publisher_mailbox_size: usize,
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
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

impl Settings {
    pub fn load() -> Result<Settings, config::ConfigError> {
        configuration::load_configuration::<Settings>()
    }
}
