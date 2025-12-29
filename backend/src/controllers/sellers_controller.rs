use axum::extract::State;
use axum::Json;
use axum::http::StatusCode;
use axum::extract::Path;

use crate::models::dto::SellerDto;
use crate::state::AppState;

pub async fn generate_sellers(State(state): State<AppState>, Path(count): Path<i64>) -> Result<Json<bool>, StatusCode> {
    match state.repos.sellers.generate_sellers(count).await {
        Ok(succeeded) => Ok(Json(succeeded)),
        Err(e) => { 
            log::error!("Error generating sellers: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_seller_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<SellerDto>, StatusCode> {
    match state.repos.sellers.get_seller(id).await {
        Ok(Some(row)) => Ok(Json(row.into())),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            log::error!("Error fetching seller by id {}: {}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}