use common::client::cache_redis::Cache;
use std::sync::Arc;
#[derive(Debug)]
pub struct AppContext {
    pub(crate) cache: Arc<Cache>,
}

impl AppContext {
    pub fn cache(&self) -> &Cache {
        &self.cache
    }
}
