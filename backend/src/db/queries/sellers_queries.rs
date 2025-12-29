use futures::FutureExt;
use crate::MySQLController;
use crate::db::db_controller::DbError;
use crate::models::row::seller::SellerRow;

impl MySQLController {
    pub async fn generate_sellers(&self, count: i64) -> Result<bool, DbError> {
        let insert_sql: &str = r#"
            INSERT INTO sellers (name, avatar_url)
            WITH RECURSIVE seq AS (
                SELECT 1 AS n
                UNION ALL
                SELECT n + 1 FROM seq WHERE n < ?
            )
            SELECT
                CONCAT('Seller ', n),
                'https://cdn.britannica.com/87/2087-050-8B2A01CD/Mona-Lisa-oil-wood-panel-Leonardo-da.jpg' AS avatar_url
            FROM seq;
            "#;

        self.with_conn(|conn| {
            async move {
                if count <= 0 {
                    return Ok(false);
                }

                sqlx::query(insert_sql)
                    .bind(count)
                    .execute(conn)
                    .await?;

                Ok(true)
            }
            .boxed()
        })
        .await
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