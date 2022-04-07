use crate::error::InternalError;
use actix::Message;
use chrono::Utc;
use cloudevents::{Data, EventBuilder};
use serde::{Deserialize, Serialize};
use serde_json::json;

use self::auth::prelude::*;

use super::EventMessage;

pub mod auth;
pub mod tenant;
pub mod user;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Message)]
#[rtype(result = "Result<(), std::io::Error>")]
pub enum Event {
    AuthSendOtp(EventMessage<auth::SendOtpMessage>),
    AuthUserCreated(EventMessage<auth::UserCreatedMessage>),
}

impl TryFrom<Event> for cloudevents::Event {
    type Error = InternalError;

    fn try_from(value: Event) -> Result<cloudevents::Event, Self::Error> {
        let builder = cloudevents::event::EventBuilderV10::new().time(Utc::now());

        let builder = match value {
            Event::AuthUserCreated(EventMessage { meta, payload }) => builder
                .source(meta.source)
                .subject(SERVICE_AUTH_SUBJECT)
                .ty(SERVICE_AUTH_EVENT_USER_CREATED)
                .id(meta.trace_id)
                .data(mime::APPLICATION_JSON.to_string(), json!(payload)),
            Event::AuthSendOtp(EventMessage { meta, payload }) => builder
                .source(meta.source)
                .subject(SERVICE_AUTH_SUBJECT)
                .ty(SERVICE_AUTH_COMMAND_SEND_OTP)
                .id(meta.trace_id)
                .data(mime::APPLICATION_JSON.to_string(), json!(payload)),
        };

        builder.build().map_err(|_| InternalError::EventBuilder)
    }
}

impl TryFrom<cloudevents::Event> for Event {
    type Error = InternalError;

    fn try_from(event: cloudevents::Event) -> Result<Event, Self::Error> {
        event
            .data()
            .and_then(|data| match data {
                Data::Json(json) => serde_json::from_value(json.clone()).ok(),
                _ => None,
            })
            .and_then(|data| match serde_json::from_value::<Event>(data) {
                Ok(e) => Some(e),
                _ => None,
            })
            .ok_or_else(|| InternalError::EventParse)
    }
}
