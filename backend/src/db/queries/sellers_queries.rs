use log::error;
use crate::models::AppState;

pub async fn generate_sellers(app_state: &AppState) -> bool {
    match app_state
        .tedooodb_pool
        .exec("INSERT INTO sellers (name, contact_info)
WITH RECURSIVE seq AS (
  SELECT 1 AS n
  UNION ALL
  SELECT n + 1 FROM seq WHERE n < 1000
)
SELECT
  CONCAT('Seller ', n),
  CONCAT('seller', n, '@example.com')
FROM seq;")
        .await
    {
        Ok(_) => true,
        Err(e) => {
            error!("Failed to execute health check query on tedooo DB: {}", e);
            false
        }
    }
}