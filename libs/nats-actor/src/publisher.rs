// Adopted from https://github.com/WuerthPhoenix/tornado/blob/develop/tornado/common/src/actors/nats_publisher.rs

use actix::prelude::*;
use async_nats::Connection;
use cloudevents::AttributesReader;
use log::*;
use serde::{Deserialize, Serialize};
use std::{io::Error, ops::Deref, rc::Rc};
use tokio::time;
use tracing_futures::Instrument;

use crate::{
    connect_with_retry,
    EventMessage,
    InternalError,
    NatsClientSettings,
    NATS_CONNECTION_RETRY_INTERVAL_SECS,
};

pub struct NatsPublisher {
    config: NatsPublisherConfig,
    nats_connection: Rc<Option<Connection>>,
    restarted: bool,
}

impl actix::io::WriteHandler<Error> for NatsPublisher {}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NatsPublisherConfig {
    pub client_settings: NatsClientSettings,
    pub subject: String,
    pub mailbox_size: usize,
}

impl NatsPublisher {
    pub async fn start_new(
        config: NatsPublisherConfig,
    ) -> Result<Addr<NatsPublisher>, InternalError> {
        Ok(actix::Supervisor::start(
            move |ctx: &mut Context<NatsPublisher>| {
                ctx.set_mailbox_capacity(config.mailbox_size);
                NatsPublisher {
                    config,
                    nats_connection: Rc::new(None),
                    restarted: false,
                }
            },
        ))
    }
}

impl Actor for NatsPublisher {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!(
            "Connecting to NATS server: {:?}",
            self.config.client_settings.addresses
        );

        let client_config = self.config.client_settings.clone();
        let nats_connection = self.nats_connection.clone();
        let restarted = self.restarted;
        ctx.wait(
            async move {
                if restarted {
                    info!(
                        "NatsPublisher was restarted after a failure. Waiting {} seconds before \
                         proceeding ...",
                        NATS_CONNECTION_RETRY_INTERVAL_SECS
                    );
                    time::sleep(time::Duration::from_secs(
                        NATS_CONNECTION_RETRY_INTERVAL_SECS,
                    ))
                    .await;
                }
                if let Some(connection) = nats_connection.deref() {
                    connection.close().await.unwrap();
                    match connection.close().await {
                        Ok(()) => {
                            debug!(
                                "NatsPublisher successfully closed previously opened NATS \
                                 connection."
                            );
                        }
                        Err(err) => {
                            error!(
                                "NatsPublisher error while closing previously opened NATS \
                                 connection. Err: {:?}",
                                err
                            )
                        }
                    };
                }
                connect_with_retry(&client_config).await
            }
            .into_actor(self)
            .map(move |client, act, ctx| match client {
                Ok(client) => {
                    info!(
                        "NatsPublisher connected to server [{:?}]",
                        &act.config.client_settings.addresses
                    );
                    act.nats_connection = Rc::new(Some(client));
                }
                Err(err) => {
                    act.nats_connection = Rc::new(None);
                    warn!("NatsPublisher connection failed. Err: {}", err);
                    ctx.stop();
                }
            }),
        );
    }
}

impl actix::Supervised for NatsPublisher {
    fn restarting(&mut self, _ctx: &mut Context<NatsPublisher>) {
        info!("Restarting NatsPublisher");
        self.restarted = true;
    }
}

impl Handler<EventMessage> for NatsPublisher {
    type Result = Result<(), InternalError>;

    fn handle(&mut self, msg: EventMessage, ctx: &mut Context<Self>) -> Self::Result {
        let trace_id = msg.event.id();
        let span = tracing::error_span!("NatsPublisher", trace_id).entered();

        trace!(
            "NatsPublisher handling Event to be sent to Nats - {:?}",
            &msg.event
        );

        let address = ctx.address();

        if let Some(connection) = self.nats_connection.deref() {
            let event =
                serde_json::to_vec(&msg.event).map_err(|err| InternalError::SerdeError {
                    cause: format! {"{}", err},
                })?;

            let client = connection.clone();
            let config = self.config.clone();

            actix::spawn(
                async move {
                    debug!("NatsPublisher publishing event to NATS");
                    match client.publish(&config.subject, &event).await {
                        Ok(_) => trace!(
                            "NatsPublisher publish event to NATS succeeded. Event: {:?}",
                            &msg
                        ),
                        Err(e) => {
                            error!("NatsPublisher error sending event to NATS. Err: {:?}", e);
                            time::sleep(time::Duration::from_secs(1)).await;
                            address.try_send(msg).unwrap_or_else(|err| {
                                error!(
                                    "NatsPublisherActor -  Error while sending event to itself. \
                                     Error: {}",
                                    err
                                )
                            });
                        }
                    }
                }
                .instrument(span.exit()),
            );
        } else {
            warn!(
                "NatsPublisher processing event but NATS connection not yet established. Stopping \
                 actor and reprocessing the event ..."
            );
            ctx.stop();
            address.try_send(msg).unwrap_or_else(|err| {
                error!(
                    "NatsPublisher error while sending event to itself. Err: {:?}",
                    err
                )
            });
        }

        Ok(())
    }
}
