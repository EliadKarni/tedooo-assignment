use sqlx::mysql::MySqlPool;
use sqlx::Error;

/// Fetches a list of products from the database with optional pagination.
///
/// # Arguments
/// - `pool`: The MySQL connection pool.
/// - `limit`: The maximum number of products to fetch.
/// - `cursor`: An optional cursor for pagination.
///
/// # Returns
/// A vector of products or an error.
pub async fn fetch_products(pool: &MySqlPool, limit: u32, cursor: Option<u32>) -> Result<Vec<Product>, Error> {
    let query = match cursor {
        Some(cursor) => {
            sqlx::query_as!(
                Product,
                "SELECT * FROM products WHERE id > ? ORDER BY id LIMIT ?",
                cursor,
                limit
            )
        }
        None => {
            sqlx::query_as!(
                Product,
                "SELECT * FROM products ORDER BY id LIMIT ?",
                limit
            )
        }
    };

    query.fetch_all(pool).await
}

/// Fetches a product by its ID from the database.
///
/// # Arguments
/// - `pool`: The MySQL connection pool.
/// - `id`: The ID of the product to fetch.
///
/// # Returns
/// The product or an error.
pub async fn fetch_product_by_id(pool: &MySqlPool, id: u32) -> Result<Product, Error> {
    sqlx::query_as!(
        Product,
        "SELECT * FROM products WHERE id = ?",
        id
    )
    .fetch_one(pool)
    .await
}