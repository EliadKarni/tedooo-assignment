use serde::Deserialize;

/// Query params for cursor-based pagination
#[derive(Deserialize)]
pub struct ProductsListParams {
    pub cursor: Option<String>,
    pub limit: Option<u32>,
}