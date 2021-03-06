use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};

use crate::error::InternalError;
use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Client,
    Database,
};
use tracing::info;

use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MongoClientSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub database_name: String,
}

#[derive(Debug, Deserialize)]
pub struct MongoClientSecrets {
    pub user_name: String,
    pub password: Secret<String>,
}

pub async fn connect(
    app_name: &str,
    config: &MongoClientSettings,
    secrets: &MongoClientSecrets,
) -> Result<Database, InternalError> {
    let mongo_server_url = format!(
        "mongodb://{}:{}@{}:{}",
        secrets.user_name,
        secrets.password.expose_secret(),
        config.host,
        config.port,
    );

    // Parse the uri now.
    let mut client_options = ClientOptions::parse(&mongo_server_url).await?;

    // Manually set an option.
    client_options.app_name = Some(app_name.to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    info!("Connecting to MongoDB...");

    let db = client.database(&config.database_name);
    ping(&db).await?;

    info!("Connected to MongoDB");
    Ok(db)
}

pub async fn ping(db: &Database) -> Result<Document, InternalError> {
    Ok(db.run_command(doc! { "ping": 1 }, None).await?)
}
