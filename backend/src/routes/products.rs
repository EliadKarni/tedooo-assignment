
use axum::Router;
use crate::models::AppState;
use crate::controllers::products_controller::{generate_products}; //, list_products, get_product_by_id};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/generate-products", axum::routing::put(generate_products))
        //.route("/products", axum::routing::get(list_products))
        //.route("/products/:id", axum::routing::get(get_product_by_id))
}
