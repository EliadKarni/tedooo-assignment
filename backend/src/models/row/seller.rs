use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SellerRow {
    pub id: i64,
    pub name: String,
    pub avatar_url: String,
}