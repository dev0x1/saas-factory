use std::fmt::{self, Debug};

use deadpool_redis::{Config as RedisConfig, Connection, Pool, Runtime};
use redis::{AsyncCommands, FromRedisValue, RedisResult, ToRedisArgs};
use tracing::info;

use crate::error::InternalError;

use serde_aux::field_attributes::deserialize_number_from_string;

pub type CachePool = Pool;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct RedisClientSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

pub fn connect(config: &RedisClientSettings) -> Result<CachePool, InternalError> {
    let redis_server_url = format!("redis://{}:{}", config.host, config.port,);

    let redis_config = RedisConfig {
        url: Some(redis_server_url),
        connection: None,
        pool: None,
    };

    redis_config
        .create_pool(Some(Runtime::Tokio1))
        .map_err(|_| InternalError::CacheClientConnectionError {
            cause: "unexpected error".to_string(),
        })
}

#[derive(Clone)]
pub struct Cache {
    pool: CachePool,
}

impl fmt::Debug for Cache {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cache")
    }
}

impl Cache {
    pub const fn new(pool: CachePool) -> Self {
        Self { pool }
    }

    async fn connection(&self) -> Result<Connection, InternalError> {
        Ok(self.pool.get().await?)
    }

    pub async fn get<T>(&self, key: &str) -> Result<Option<T>, InternalError>
    where
        T: FromRedisValue + Debug + Send + Sync,
    {
        let mut cache = self.connection().await?;

        if self.exists(key).await? {
            let result = cache.get(&key).await;
            if let Ok(inner) = result {
                info!("HIT | {key} | {inner:#?}");
                return Ok(Some(inner));
            }
        }

        info!("MISS | {key}");
        Ok(None)
    }

    pub async fn set<T>(&self, key: &str, value: T, expiry: usize) -> Result<(), InternalError>
    where
        T: ToRedisArgs + Debug + Send + Sync,
    {
        let mut cache = self.connection().await?;

        info!("SET | {key} | {value:#?}");
        let result = cache.set_ex::<_, _, ()>(&key, value, expiry).await;

        if result.is_err() {
            info!("Failed to set");
        }
        Ok(())
    }

    pub async fn delete(&self, key: &str) -> Result<(), InternalError> {
        let mut cache = self.connection().await?;

        info!("DELETE | {key}");
        let result = cache.del::<_, ()>(&key).await;

        if result.is_err() {
            info!("Failed to delete");
        }
        Ok(())
    }

    pub async fn exists(&self, key: &str) -> Result<bool, InternalError> {
        let mut cache = self.connection().await?;

        let cache_exists: RedisResult<bool> = cache.exists(&key).await;
        if let Ok(exists) = cache_exists {
            info!("EXISTS | {key} | {exists}");
            return Ok(exists);
        }

        info!("EXISTS | {key} | false");
        Ok(false)
    }
}
