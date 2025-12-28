use crate::models::dto::HealthStatus;
use crate::state::AppState;
use axum::extract::State;
use axum::Json;
use axum::http::StatusCode;


pub async fn get_health(State(state): State<AppState>) -> Result<Json<HealthStatus>, StatusCode> {
    // Check tedoooDB connectivity
    match state.health().await{
        Ok(health_status) => Ok(Json( health_status )),
        Err(e) => {
            log::error!("Health check failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
