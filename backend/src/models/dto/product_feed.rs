use serde::{Deserialize, Serialize};

use crate::models::dto::product::ProductDto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductFeedResponse {
    pub items: Vec<ProductDto>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductCursor {
    /// Milliseconds since Unix epoch (UTC)
    pub created_at_ms: i64,
    pub id: i64,
}