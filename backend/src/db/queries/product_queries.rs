use log::error;
use crate::models::AppState;

pub async fn generate_products(app_state: &AppState) -> bool {
    match app_state
        .tedooodb_pool
        .exec(r#"
            INSERT INTO products (name, description, price, seller_id)
            SELECT CONCAT('Product ', n), CONCAT('Description for product ', n), ROUND((RAND() * 490 + 10), 2), 1 + FLOOR(RAND() * 1000)
            FROM (
                SELECT ROW_NUMBER() OVER () AS n
                FROM information_schema.columns c
                CROSS JOIN information_schema.tables t
                LIMIT 1000
            )
            AS seq;
        "#).await
    {
        Ok(_) => true,
        Err(e) => {
            error!("Failed to execute health check query on tedooo DB: {}", e);
            false
        }
    }
}