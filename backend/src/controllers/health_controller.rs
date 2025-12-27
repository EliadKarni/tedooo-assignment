use crate::models::dto::HealthStatus;
use crate::state::AppState;
use axum::extract::State;
use axum::Json;


pub async fn get_health(State(state): State<AppState>) -> Json<HealthStatus> {
    // Check tedoooDB connectivity
    let health_status = state.health().await;
    //let db_ok = state.repos.is_alive().await;
    Json( health_status)
}
