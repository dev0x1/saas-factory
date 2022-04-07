use crate::settings::Settings;
use common::{
    client::{cache_redis::RedisClientSecrets, sm_vault},
    error::InternalError,
};
use secrecy::Secret;
use serde::Deserialize;
use vaultrs::client::VaultClient;

#[derive(Debug, Deserialize)]
pub struct Secrets {
    pub cache: RedisClientSecrets,
    pub smtp: SmtpClientSecrets,
}

#[derive(Debug, Deserialize)]
pub struct SmtpClientSecrets {
    pub user_name: String,
    pub password: Secret<String>,
}

pub async fn read(settings: &Settings) -> Result<Secrets, InternalError> {
    let vault_client: VaultClient = sm_vault::connect(&settings.vault)?;

    let cache_secrets: RedisClientSecrets =
        sm_vault::get_secret_value(&vault_client, &settings.cache_secrets_path).await?;
    let smtp_secrets: SmtpClientSecrets =
        sm_vault::get_secret_value(&vault_client, &settings.smtp_secrets_path).await?;

    Ok(Secrets {
        cache: cache_secrets,
        smtp: smtp_secrets,
    })
}
