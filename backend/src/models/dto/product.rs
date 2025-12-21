use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductDto {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub created_at: String,
    pub seller_id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SellerDto {
    pub id: u64,
    pub name: String,
    pub contact_info: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductWithSellerDto {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub created_at: String,
    pub seller: SellerDto,
}
