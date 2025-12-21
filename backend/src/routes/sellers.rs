use axum::Router;
use crate::models::AppState;
use crate::controllers::sellers_controller::generate_sellers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/generate-sellers", axum::routing::put(generate_sellers))
}
