use serde::{Serialize, Deserialize};
use crate::models::row::seller::SellerRow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellerDto {
    pub id: i64,
    pub name: String,
    pub avatar_url: String,
}

impl From<SellerRow> for SellerDto {
    fn from(r: SellerRow) -> Self {
        Self {
            id: r.id,
            name: r.name,
            avatar_url: r.avatar_url,
        }
    }
}