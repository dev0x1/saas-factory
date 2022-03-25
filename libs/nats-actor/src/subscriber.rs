use crate::{connect_with_retry, InternalError, NatsClientSettings};

use actix::prelude::*;
use async_nats::{Connection, Message as NatsMessage};
use futures_util::stream;
use log::*;
use serde::{Deserialize, Serialize};

#[derive(Message, Debug)]
#[rtype(result = "Result<(), InternalError>")]
pub struct NatsStreamMessage {
    pub msg: NatsMessage,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct NatsSubscriberConfig {
    pub client_settings: NatsClientSettings,
    pub subject: String,
    pub mailbox_size: usize,
}

pub async fn subscribe_to_nats<
    F: 'static + FnMut(NatsStreamMessage) -> Result<(), InternalError> + Sized + Unpin,
>(
    config: NatsSubscriberConfig,
    callback: F,
) -> Result<(), InternalError> {
    let client = connect_with_retry(&config.client_settings).await?;

    let subscription = client.subscribe(&config.subject).await.map_err(|err| {
        InternalError::NatsOperationError {
            cause: format! {"Cannot subscribe to subject [{}]. Err: {:?}", config.subject, err},
        }
    })?;

    info!("Subscribed to subject [{}]", config.subject);

    let message_stream = stream::unfold(subscription, |sub| async {
        sub.next().await.map(|msg| (NatsStreamMessage { msg }, sub))
    });

    NatsSubscriber::create(|ctx| {
        ctx.set_mailbox_capacity(config.mailbox_size);
        ctx.add_message_stream(message_stream);
        NatsSubscriber { callback, client }
    });

    Ok(())
}

struct NatsSubscriber<F>
where
    F: 'static + FnMut(NatsStreamMessage) -> Result<(), InternalError> + Sized + Unpin,
{
    callback: F,
    // The client must live as long as the actor, otherwise the connection is dropped when the
    // client is deallocated
    #[allow(dead_code)]
    client: Connection,
}

impl<F> Actor for NatsSubscriber<F>
where
    F: 'static + FnMut(NatsStreamMessage) -> Result<(), InternalError> + Sized + Unpin,
{
    type Context = Context<Self>;
}

impl<F> Handler<NatsStreamMessage> for NatsSubscriber<F>
where
    F: 'static + FnMut(NatsStreamMessage) -> Result<(), InternalError> + Sized + Unpin,
{
    type Result = Result<(), InternalError>;

    fn handle(&mut self, msg: NatsStreamMessage, _: &mut Context<Self>) -> Self::Result {
        trace!("Message received");
        if let Err(err) = (&mut self.callback)(msg) {
            error!("Received message processing failed: {:?}", err);
            Err(err)
        } else {
            Ok(())
        }
    }
}
