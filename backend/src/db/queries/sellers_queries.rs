use futures::FutureExt;
use log::error;
use crate::MySQLController;
use crate::db::db_controller::DbError;
use crate::models::row::seller::SellerRow;

impl MySQLController {
    pub async fn generate_sellers(&self) -> bool {
        match self.exec(
        r#"
            INSERT INTO sellers (name, avatar_url)
            WITH RECURSIVE seq AS (
                SELECT 1 AS n
                UNION ALL
                SELECT n + 1 FROM seq WHERE n < 1000
            )
            SELECT
                CONCAT('Seller ', n),
                'https://cdn.britannica.com/87/2087-050-8B2A01CD/Mona-Lisa-oil-wood-panel-Leonardo-da.jpg' AS avatar_url
            FROM seq;
            "#
        ).await {
            Ok(_) => true,
            Err(e) => {
                error!("Failed to execute health check query on tedooo DB: {}", e);
                false
            }
        }
    }

    pub async fn get_seller_by_id(&self, id: i64) -> Result<Option<SellerRow>, DbError> {
        let sql = r#"
            SELECT *
            FROM sellers
            WHERE id = ?
            LIMIT 1
        "#;

        self.with_conn(|conn| {
            async move {
                sqlx::query_as::<_, SellerRow>(sql)
                    .bind(id)
                    .fetch_optional(conn)
                    .await
            }
            .boxed()
        }).await
    }
}