use axum::{Json, extract::Query};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the pagination parameters for the `GET /products` endpoint.
///
/// - `cursor`: An optional cursor for pagination.
/// - `limit`: An optional limit for the number of products to fetch.
#[derive(Deserialize)]
pub struct Pagination {
    pub cursor: Option<String>,
    pub limit: Option<u32>,
}

/// Represents a product in the system.
///
/// - `id`: The unique identifier of the product.
/// - `name`: The name of the product.
/// - `description`: A brief description of the product.
/// - `price`: The price of the product.
#[derive(Serialize)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub price: f64,
}

/// Handles the `GET /products` endpoint.
///
/// This endpoint retrieves a list of products with optional pagination.
///
/// # Parameters
/// - `pagination`: The pagination parameters extracted from the query string.
///
/// # Returns
/// A JSON response containing a list of products.
///
/// # Example
/// ```json
/// [
///   {
///     "id": 1,
///     "name": "Sample Product",
///     "description": "This is a sample product.",
///     "price": 19.99
///   }
/// ]
/// ```
pub async fn get_products(Query(pagination): Query<Pagination>) -> Json<Vec<Product>> {
    // Placeholder logic for fetching products
    let products = vec![
        Product {
            id: 1,
            name: "Sample Product".to_string(),
            description: "This is a sample product.".to_string(),
            price: 19.99,
        },
    ];

    Json(products)
}