use std::time::Duration;

use actix::prelude::Message;
use async_nats::{Connection, Options};
use backoff::{future::retry, ExponentialBackoff};

use cloudevents::Event as CloudEvent;
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

const NATS_CONNECTION_RETRY_INTERVAL_SECS: u64 = 10;

pub mod publisher;
pub mod subscriber;

#[derive(Clone, Debug, Display, Error)]
pub enum InternalError {
    #[display(fmt = "Nats server connection failed: {address}")]
    NatsServerConnectionError { address: String },
    #[display(fmt = "Nats operation failed: {cause}")]
    NatsOperationError { cause: String },
    #[display(fmt = "Failed while ser-de Nats message {cause}")]
    SerdeError { cause: String },
    #[display(fmt = "Error: {}", cause)]
    GenericError { cause: String },
}

#[derive(Message, Debug)]
#[rtype(result = "Result<(), InternalError>")]
pub struct EventMessage {
    pub event: CloudEvent,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NatsClientSettings {
    pub addresses: Vec<String>,
    pub max_reconnects: Option<usize>,
    pub retry_timeout: Option<Duration>,
}

fn backoff(timeout: Option<Duration>) -> ExponentialBackoff {
    ExponentialBackoff {
        max_elapsed_time: timeout,
        ..Default::default()
    }
}

pub async fn connect_with_retry(config: &NatsClientSettings) -> Result<Connection, InternalError> {
    info!("Connecting to NATS...");
    let addresses = config.addresses.join(",");

    let connect_op = || async {
        let options = Options::new()
            .disconnect_callback(|| error!("connection lost"))
            .max_reconnects(config.max_reconnects);

        Ok(options.connect(&addresses).await?)
    };

    retry(backoff(config.retry_timeout), connect_op)
        .await
        .map_err(|_| InternalError::NatsServerConnectionError { address: addresses })
}

pub async fn connect(config: &NatsClientSettings) -> Result<Connection, InternalError> {
    info!("Connecting to NATS...");
    let addresses = config.addresses.join(",");

    let options = Options::new()
        .disconnect_callback(|| error!("connection lost"))
        .max_reconnects(config.max_reconnects);

    options
        .connect(&addresses)
        .await
        .map_err(|_| InternalError::NatsServerConnectionError { address: addresses })
}
