use mongodb::Database;
use std::sync::Arc;

/// The AppContext contains all the global data commonly used in the vast
/// majority of request handlers.
#[derive(Debug)]
pub struct AppContext {
    pub(crate) db: Arc<Database>,
}

impl AppContext {
    /// A MongoDB reference to the underlying database. Used to interract with
    /// collections, etc.
    pub fn db(&self) -> &Database {
        &self.db
    }
}
