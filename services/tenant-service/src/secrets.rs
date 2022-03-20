use crate::settings::Settings;
use common::{
    client::{cache_redis::RedisClientSecrets, db_mongo::MongoClientSecrets, sm_vault},
    error::InternalError,
};
use serde::Deserialize;
use vaultrs::client::VaultClient;

#[derive(Clone, Debug, Deserialize)]
pub struct Secrets {
    pub cache: RedisClientSecrets,
    pub db: MongoClientSecrets,
}

pub async fn read(settings: &Settings) -> Result<Secrets, InternalError> {
    let vault_client: VaultClient = sm_vault::connect(&settings.vault).await?;

    let cache_secrets: RedisClientSecrets =
        sm_vault::get_secret_value(&vault_client, &settings.cache_secrets_path).await?;

    let db_secrets: MongoClientSecrets =
        sm_vault::get_secret_value(&vault_client, &settings.db_secrets_path).await?;

    Ok(Secrets {
        cache: cache_secrets,
        db: db_secrets,
    })
}
