use common::utils::configuration;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub log: LogSettings,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub base_url: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub workers: usize,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct LogSettings {
    pub level: String,
}

pub fn get_settings() -> Result<Settings, config::ConfigError> {
    configuration::load_configuration::<Settings>()
}
