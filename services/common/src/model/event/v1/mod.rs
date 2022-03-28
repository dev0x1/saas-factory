use crate::error::InternalError;
use chrono::Utc;
use cloudevents::{Data, EventBuilder};
use serde::{Deserialize, Serialize};
use serde_json::json;

use self::auth::prelude::*;

pub mod auth;
pub mod tenant;
pub mod user;

pub const SERVICE_AUTH_SUBJECT: &str = "service.auth";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EventMetadata {
    source: String,
    trace_id: String,
}

impl EventMetadata {
    pub fn new(source: String, trace_id: &str) -> EventMetadata {
        EventMetadata {
            source,
            trace_id: trace_id.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Event {
    AuthUserCreated(auth::UserCreated),
}

impl TryFrom<Event> for cloudevents::Event {
    type Error = InternalError;

    fn try_from(value: Event) -> Result<cloudevents::Event, Self::Error> {
        let builder = cloudevents::event::EventBuilderV10::new().time(Utc::now());

        let payload = json!(value);
        let builder = match value {
            Event::AuthUserCreated(auth::UserCreated { meta, payload }) => builder
                .source(meta.source)
                .subject(SERVICE_AUTH_SUBJECT)
                .ty(SERVICE_AUTH_EVENT_USER_CREATED)
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
