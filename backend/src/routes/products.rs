
use axum::Router;
use crate::state::AppState;
use crate::controllers::products_controller::{generate_products, get_product_by_id, get_products_feed}; 

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/generate-products/{count}", axum::routing::put(generate_products))
        .route("/products", axum::routing::get(get_products_feed))
        .route("/products/{id}", axum::routing::get(get_product_by_id))
}
