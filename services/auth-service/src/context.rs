use actix::Addr;
use common::client::cache_redis::Cache;
use mongodb::Database;
use nats_actor::publisher::NatsPublisher;
use std::sync::Arc;

/// The AppContext contains all the global data commonly used in the vast
/// majority of request handlers.
#[derive(Debug)]
pub struct AppContext {
    pub(crate) db: Arc<Database>,
    pub(crate) cache: Arc<Cache>,
    pub(crate) event_publisher: Arc<Addr<NatsPublisher>>,
}

impl AppContext {
    /// A MongoDB reference to the underlying database. Used to interract with
    /// collections, etc.
    pub fn db(&self) -> &Database {
        &self.db
    }

    /// A Redis cache pool.
    pub fn cache(&self) -> &Cache {
        &self.cache
    }

    /// A Redis cache pool.
    pub fn event_publisher(&self) -> &Addr<NatsPublisher> {
        &self.event_publisher
    }
}
