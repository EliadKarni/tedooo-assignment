use axum::{Json, extract::Path};
use serde::Serialize;

/// Represents the details of a product.
///
/// - `id`: The unique identifier of the product.
/// - `name`: The name of the product.
/// - `description`: A brief description of the product.
/// - `price`: The price of the product.
/// - `seller`: Information about the seller of the product.
#[derive(Serialize)]
pub struct ProductDetails {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub seller: SellerInfo,
}

/// Represents the information about a seller.
///
/// - `id`: The unique identifier of the seller.
/// - `name`: The name of the seller.
/// - `contact_info`: The contact information of the seller.
#[derive(Serialize)]
pub struct SellerInfo {
    pub id: u32,
    pub name: String,
    pub contact_info: String,
}

/// Handles the `GET /products/{id}` endpoint.
///
/// This endpoint retrieves the details of a specific product, including seller information.
///
/// # Arguments
/// - `id`: The ID of the product to retrieve.
///
/// # Returns
/// A JSON response containing the product details.
pub async fn get_product_details(Path(id): Path<u32>) -> Json<ProductDetails> {
    // Placeholder logic for fetching product details
    let product_details = ProductDetails {
        id,
        name: "Sample Product".to_string(),
        description: "This is a sample product.".to_string(),
        price: 19.99,
        seller: SellerInfo {
            id: 1,
            name: "Sample Seller".to_string(),
            contact_info: "seller@example.com".to_string(),
        },
    };

    Json(product_details)
}