use serde::{Deserialize, Serialize};

/// Data Transfer Object (DTO) for a product.
///
/// This struct represents the data structure for transferring product information
/// between different layers of the application.
#[derive(Serialize, Deserialize)]
pub struct ProductDTO {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub price: f64,
}

/// Data Transfer Object (DTO) for a seller.
///
/// This struct represents the data structure for transferring seller information
/// between different layers of the application.
#[derive(Serialize, Deserialize)]
pub struct SellerDTO {
    pub id: u32,
    pub name: String,
    pub contact_info: String,
}

/// Data Transfer Object (DTO) for pagination parameters.
///
/// This struct represents the data structure for transferring pagination parameters
/// between different layers of the application.
#[derive(Deserialize)]
pub struct PaginationDTO {
    pub cursor: Option<String>,
    pub limit: Option<u32>,
}