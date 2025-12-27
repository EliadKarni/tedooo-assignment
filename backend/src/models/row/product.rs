use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductRow {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub image_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub seller_id: Option<i64>,
}