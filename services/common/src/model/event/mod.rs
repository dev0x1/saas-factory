use chrono::Utc;
use cloudevents::{Data, EventBuilder};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::error::InternalError;

pub const EVENT_TYPE_PING: &str = "com.example.ping";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PingMessage {
    pub trace_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Event {
    Ping(PingMessage),
}

impl TryFrom<Event> for cloudevents::Event {
    type Error = InternalError;

    fn try_from(value: Event) -> Result<cloudevents::Event, Self::Error> {
        let builder = cloudevents::event::EventBuilderV10::new()
            .source("http://localhost")
            .time(Utc::now());

        let payload = json!(value);
        let builder = match value {
            Event::Ping(PingMessage { trace_id }) => builder
                .subject("ping_message")
                .ty(EVENT_TYPE_PING)
                .id(trace_id)
                .data(mime::APPLICATION_JSON.to_string(), payload),
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
