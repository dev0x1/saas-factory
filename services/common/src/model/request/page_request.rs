use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Clone, Validate)]
pub struct PageRequest {
    #[validate(range(min = 0))]
    #[serde(default = "default_page")]
    pub page: u64,

    #[validate(range(min = 1))]
    #[serde(default = "default_page_size", rename = "pageSize")]
    pub page_size: u64,
}

fn default_page_size() -> u64 {
    10
}

fn default_page() -> u64 {
    0
}
