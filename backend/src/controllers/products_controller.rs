use axum::extract::State;
use axum::Json;
use axum::{extract::{Query, Path}, response::IntoResponse, http::StatusCode};

use crate::db::generate_products as generate_products_db;
use crate::models::AppState;
use crate::models::dto::product_list_params::ProductsListParams;

use crate::models::dto::product::{ProductDto, ProductWithSellerDto, SellerDto};
use crate::cache::redis_controller::RedisCache;
use sqlx::Row;
use std::time::Duration;


pub async fn generate_products(State(state): State<AppState>) -> Json<bool> {
    let succeeded: bool = generate_products_db(&state).await;
    Json(succeeded)
}
/*
/// GET /products - List products with cursor-based pagination and Redis cache
pub async fn list_products(
    State(state): State<AppState>,
    Query(params): Query<ProductsListParams>,
) -> impl IntoResponse {
    let limit = params.limit.unwrap_or(20).min(100);
    let cursor = params.cursor.clone();
    let cache_key = format!("products:{}:{}", cursor.clone().unwrap_or_default(), limit);
    let redis = RedisCache::new();

    // Try cache
    if let Some(cached) = redis.get::<serde_json::Value>(&cache_key).await {
        return (StatusCode::OK, Json(cached));
    }

    // Decode cursor
    let (created_at, id) = match cursor {
        Some(c) => {
            match base64::decode(c) {
                Ok(bytes) => {
                    let s = String::from_utf8_lossy(&bytes);
                    let parts: Vec<&str> = s.split('|').collect();
                    if parts.len() == 2 {
                        (parts[0].to_string(), parts[1].parse::<u64>().unwrap_or(0))
                    } else {
                        ("1970-01-01 00:00:00".to_string(), 0)
                    }
                }
                Err(_) => ("1970-01-01 00:00:00".to_string(), 0)
            }
        }
        None => ("1970-01-01 00:00:00".to_string(), 0)
    };

    // Query DB
    let query = r#"
        SELECT id, name, description, price, created_at, seller_id
        FROM products
        WHERE (created_at, id) > (?, ?)
        ORDER BY created_at ASC, id ASC
        LIMIT ?
    "#;
    let rows = match state.tedooodb_pool.query(query, &[&created_at, &id, &limit]).await {
        Ok(rows) => rows,
        Err(_) => return (StatusCode::SERVICE_UNAVAILABLE, Json(json_error("DB unavailable"))),
    };

    let mut products: Vec<ProductDto> = vec![];
    for row in rows {
        products.push(ProductDto {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            price: row.get("price"),
            created_at: row.get("created_at"),
            seller_id: row.get("seller_id"),
        });
    }

    // Next cursor
    let next_cursor = products.last().map(|p| base64::encode(format!("{}|{}", p.created_at, p.id)));
    let resp = serde_json::json!({ "products": products, "next_cursor": next_cursor });
    let _ = redis.set(&cache_key, &resp, Duration::from_secs(30)).await;
    (StatusCode::OK, Json(resp))
}


/// GET /products/{id} - Get product by ID with seller info and Redis cache
pub async fn get_product_by_id(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> impl IntoResponse {
    let cache_key = format!("product:{}", id);
    let redis = RedisCache::new(&state.tedooodb_pool.redis_url);
    if let Some(cached) = redis.get::<serde_json::Value>(&cache_key).await {
        return (StatusCode::OK, Json(cached));
    }

    let query = r#"
        SELECT p.id, p.name, p.description, p.price, p.created_at,
               s.id as seller_id, s.name as seller_name, s.contact_info as seller_contact_info
        FROM products p
        JOIN sellers s ON p.seller_id = s.id
        WHERE p.id = ?
    "#;
    let rows = match state.tedooodb_pool.query(query, &[&id]).await {
        Ok(rows) => rows,
        Err(_) => return (StatusCode::SERVICE_UNAVAILABLE, Json(json_error("DB unavailable"))),
    };
    if rows.is_empty() {
        return (StatusCode::NOT_FOUND, Json(json_error("Product not found")));
    }
    let row = &rows[0];
    let product = ProductWithSellerDto {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
        price: row.get("price"),
        created_at: row.get("created_at"),
        seller: SellerDto {
            id: row.get("seller_id"),
            name: row.get("seller_name"),
            contact_info: row.get("seller_contact_info"),
        },
    };
    let resp = serde_json::json!(product);
    let _ = redis.set(&cache_key, &resp, Duration::from_secs(300)).await;
    (StatusCode::OK, Json(resp))
}

fn json_error(msg: &str) -> serde_json::Value {
    serde_json::json!({ "error": msg })
}
    */
