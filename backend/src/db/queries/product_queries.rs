use chrono::{DateTime, NaiveDateTime, Utc};
use futures::FutureExt;
use crate::MySQLController;
use crate::models::row::product::ProductRow;
use crate::db::db_controller::DbError;
use sqlx::{Connection};

impl MySQLController {
    pub async fn generate_products(&self, count: i64) -> Result<(), DbError> {
        let insert_sql = r#"
            INSERT INTO products (title, description, price, image_url,seller_id)
            WITH RECURSIVE
            seq AS (
                SELECT 1 AS n
                UNION ALL
                SELECT n + 1 FROM seq WHERE n < ?
            ),
            seller_shuffled AS (
                SELECT
                id,
                ROW_NUMBER() OVER (ORDER BY RAND()) AS rn
                FROM sellers
            ),
            seller_cnt AS (
                SELECT COUNT(*) AS cnt FROM seller_shuffled
            )
            SELECT
            CONCAT('Product ', seq.n) AS name,
            CONCAT('Description for product ', seq.n) AS description,
            ROUND((RAND() * 490 + 10), 2) AS price,
            'https://cdn11.bigcommerce.com/s-x49po/images/stencil/1500x1500/products/62041/261586/1607601332291_IMG_20201128_134909__85629.1687170945.jpg?c=2' AS image_url,
            ss.id AS seller_id
            FROM seq
            CROSS JOIN seller_cnt sc
            JOIN seller_shuffled ss
            ON ss.rn = 1 + MOD(seq.n - 1, sc.cnt);
        "#;

        self.with_conn(|conn| {
            async move {
                if count <= 0 {
                return Ok(());
            }

            let mut tx = conn.begin().await?;


            sqlx::query(insert_sql)
                .bind(count)
                .execute(tx.as_mut())
                .await?;

            tx.commit().await?;
            Ok(())
            }
            .boxed()
        })
        .await
    }

    pub async fn get_product_by_id(&self, id: i64) -> Result<Option<ProductRow>, DbError> {
        let sql = r#"
            SELECT *
            FROM products
            WHERE id = ?
            LIMIT 1
        "#;

        self.with_conn(|conn| {
            async move {
                sqlx::query_as::<_, ProductRow>(sql)
                    .bind(id)
                    .fetch_optional(conn)
                    .await
            }
            .boxed()
        }).await
    }
    
   pub async fn list_products_feed(
        &self,
        cursor_created_at: Option<DateTime<Utc>>,
        cursor_id: Option<i64>,
        limit_plus_one: i64,
    ) -> Result<Vec<ProductRow>, DbError> {
        let sql_first_page = r#"
            SELECT *
            FROM products
            ORDER BY created_at DESC, id DESC
            LIMIT ?
        "#;

        let sql_next_page = r#"
            SELECT *
            FROM products
            WHERE
                (created_at < ?) OR (created_at = ? AND id < ?)
            ORDER BY created_at DESC, id DESC
            LIMIT ?
        "#;

        self.with_conn(move |conn| {
            async move {
                match (cursor_created_at, cursor_id) {
                    (Some(ts), Some(id)) => {
                        // MySQL comparisons typically bind as NaiveDateTime (no timezone).
                        let ts: NaiveDateTime = ts.naive_utc();

                        sqlx::query_as::<_, ProductRow>(sql_next_page)
                            .bind(ts)
                            .bind(ts)
                            .bind(id)
                            .bind(limit_plus_one)
                            .fetch_all(conn)
                            .await
                    }
                    _ => {
                        sqlx::query_as::<_, ProductRow>(sql_first_page)
                            .bind(limit_plus_one)
                            .fetch_all(conn)
                            .await
                    }
                }
            }
            .boxed()
        })
        .await
    }
    
}
