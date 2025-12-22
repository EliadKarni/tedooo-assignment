use std::time::Duration;
use log::error;
use crate::models::AppState;

pub async fn check_tedooo_db_available(app_state: &AppState) -> bool {
    match app_state
        .tedooodb_pool
        .exec_with_timeout(
            r#"
            SELECT 1
            "#, Duration::from_secs(2))
        .await
    {
        Ok(_) => true,
        Err(e) => {
            error!("Failed to execute health check query on tedooo DB: {}", e);
            false
        }
    }
}
