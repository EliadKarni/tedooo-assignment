use crate::db::check_tedooo_db_available;
use crate::models::HealthStatus;
use crate::models::AppState;
use axum::extract::State;
use axum::Json;


pub async fn get_health(State(state): State<AppState>) -> Json<HealthStatus> {
    // Check todoooDB connectivity
    let db_ok = check_tedooo_db_available(&state).await;
    Json(HealthStatus { database: db_ok, redis: false })
}
