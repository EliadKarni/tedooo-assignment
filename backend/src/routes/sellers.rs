use axum::Router;
use crate::state::AppState;
use crate::controllers::sellers_controller::{generate_sellers, get_seller_by_id};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/generate-sellers/{count}", axum::routing::put(generate_sellers))
        .route("/seller/{id}", axum::routing::get(get_seller_by_id))
}
