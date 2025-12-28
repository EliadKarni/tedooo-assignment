use axum::extract::{Path, Query, State};
use axum::Json;
use axum::http::StatusCode;
use log::error;
use serde::Deserialize;

use crate::state::AppState;
use crate::models::dto::{ProductDto, ProductFeedResponse};

#[derive(Debug, Deserialize)]
pub struct ProductsFeedQuery {
    pub limit: Option<i64>,
    pub cursor: Option<String>,
}


pub async fn generate_products(State(state): State<AppState>) -> Result<Json<bool>, StatusCode> {
    match state.repos.products.generate_products(1000).await {
        Ok(returned_value) => Ok(Json(returned_value)),
        Err(e) => {
            error!("Error generating products: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
    
}

pub async fn get_product_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ProductDto>, StatusCode> {
    match state.repos.products.get_product(id).await {
        Ok(Some(row)) => Ok(Json(row.into())),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            error!("Error fetching product by id {}: {}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_products_feed(
    State(state): State<AppState>,
    Query(q): Query<ProductsFeedQuery>,
) -> Result<Json<ProductFeedResponse>, StatusCode> {
    let limit = q.limit.unwrap_or(20).clamp(1, 100);

    match state.repos.products.list_feed(limit, q.cursor).await {
        Ok(Some(resp)) => Ok(Json(resp)),
        Ok(None) => {
            Ok(Json(ProductFeedResponse { items: vec![], next_cursor: None }))
        }
        Err(e) => {
            error!("Error fetching products feed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}