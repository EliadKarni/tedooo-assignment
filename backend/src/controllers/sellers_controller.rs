use crate::db::generate_sellers as generate_sellers_db;
use crate::models::AppState;
use axum::extract::State;
use axum::Json;


pub async fn generate_sellers(State(state): State<AppState>) -> Json<bool> {
    let succeeded: bool = generate_sellers_db(&state).await;
    Json(succeeded)
}
