use serde::{Deserialize, Serialize};

pub mod v1;

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
pub struct EventMessage<T: Clone> {
    pub meta: EventMetadata,
    pub payload: T,
}
