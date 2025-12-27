use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};
use crate::models::row::product::ProductRow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductDto {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub image_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub seller_id: Option<i64>,
}

impl From<ProductRow> for ProductDto {
    fn from(r: ProductRow) -> Self {
        Self {
            id: r.id,
            title: r.title,
            description: r.description,
            price: r.price,
            image_url: r.image_url,
            created_at: r.created_at,
            seller_id: r.seller_id,
        }
    }
}
