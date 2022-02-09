use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tracing::error;

use secrecy::Secret;
use serde_aux::field_attributes::deserialize_number_from_string;

pub type DatabasePool = sqlx::postgres::PgPool;
pub type DatabaseRow = sqlx::postgres::PgRow;
pub type QueryResult = sqlx::postgres::PgQueryResult;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct PostgresClientSettings {
    pub user_name: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub connections_per_pool: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub database_lifetime: u64,
}

pub async fn connect(config: &PostgresClientSettings) -> Result<DatabasePool, sqlx::Error> {
    let pg_server_url = format!(
        "postgresql://{}:{}@{}:{}/{}",
        config.user_name,
        config.password.expose_secret(),
        config.host,
        config.port,
        config.database_name,
    );

    PgPoolOptions::new()
        .max_connections(config.connections_per_pool)
        .max_lifetime(Duration::from_secs(config.database_lifetime))
        .connect(&pg_server_url)
        .await
        .map_err(|error| {
            error!("Failed to connect to Postgres: {error:#?}");
            error
        })
}
