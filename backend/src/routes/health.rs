use axum::Router;
use crate::models::AppState;
use crate::controllers::health_controller::get_health;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", axum::routing::get(get_health))
}


