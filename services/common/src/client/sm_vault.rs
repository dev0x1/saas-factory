use secrecy::{ExposeSecret, Secret};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::error::InternalError;
use tracing::info;
use vaultrs::{
    client::{VaultClient, VaultClientSettingsBuilder},
    kv2,
};

#[derive(Debug, Deserialize, Clone)]
pub struct VaultClientConfig {
    pub server_url: String,
    pub token: Secret<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct VaultKvPath {
    pub mount: String,
    pub path: String,
}

pub async fn connect(config: &VaultClientConfig) -> Result<VaultClient, InternalError> {
    let settings = VaultClientSettingsBuilder::default()
        .address(&config.server_url)
        .token(config.token.expose_secret())
        .build()?;

    info!("Connecting to Vault Server...");

    Ok(VaultClient::new(settings)?)
}

pub async fn get_secret_value<T>(
    client: &VaultClient,
    kv_config: &VaultKvPath,
) -> Result<T, InternalError>
where
    T: DeserializeOwned,
{
    let secret = kv2::read(client, &kv_config.mount, &kv_config.path).await?;
    Ok(secret)
}

pub async fn set_secret_value<T>(
    client: &VaultClient,
    kv_config: &VaultKvPath,
    secret: &T,
) -> Result<(), InternalError>
where
    T: Serialize,
{
    kv2::set(client, &kv_config.mount, &kv_config.path, secret).await?;
    Ok(())
}
