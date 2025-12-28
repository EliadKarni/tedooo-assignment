use std::{sync::Arc, time::Duration};

use chrono::{DateTime, Utc};

use crate::db::db_controller::DbError;
use crate::models::dto::{ProductCursor, ProductDto, ProductFeedResponse};
use crate::models::row::product::ProductRow;
use crate::utils::{decode_cursor, encode_cursor};

use super::BaseRepository;

#[derive(Clone)]
pub struct ProductRepository {
    base: Arc<BaseRepository>, // private field with private type - OK
}

impl ProductRepository {
    /// Create a new ProductRepository.
    pub(super) fn new(base: Arc<BaseRepository>) -> Self {
        Self { base }
    }

    fn get_product_key(id: i64) -> String {
        format!("products:by_id:{}", id)
    }

    pub async fn get_product(&self, id: i64) -> anyhow::Result<Option<ProductRow>> {
        let key = Self::get_product_key(id);

        // Cache stores Option<ProductRow> (Some / None)
        match self.base.cache().get::<Option<ProductRow>>(&key).await {
            Ok(Some(v)) => return Ok(v), // v is Option<ProductRow>
            Ok(None) => { /* cache miss */ }
            Err(_) => { /* cache failure treated as miss */ }
        }

        let v = self.base.db().get_product_by_id(id).await?;

        // Best-effort cache set (store Option)
        let _ = self
            .base
            .cache()
            .set(
                &key,
                &v,
                Duration::from_secs(self.base.ttl_seconds() as u64),
            )
            .await;

        Ok(v)
    }

    pub async fn generate_products(&self, count: i64) -> Result<bool, DbError> {
        match self.base.db().generate_products(count).await{
            Ok(returned_value) => Ok(returned_value),
            Err(e) => Err(e),
        }
    }

   pub async fn list_feed(
        &self,
        limit: i64,
        cursor: Option<String>,
    ) -> anyhow::Result<Option<ProductFeedResponse>> {
        let limit = limit.clamp(1, 100);
        let limit_plus_one = limit + 1;

        let decoded: Option<ProductCursor> = match cursor {
            Some(c) => Some(decode_cursor::<ProductCursor>(&c)?),
            None => None,
        };

        let cursor_ts: Option<DateTime<Utc>> = match decoded.as_ref() {
            Some(c) => {
                let ts = DateTime::<Utc>::from_timestamp_millis(c.created_at_ms)
                    .ok_or_else(|| anyhow::anyhow!("Invalid cursor timestamp"))?;
                Some(ts)
            }
            None => None,
        };

        let rows = self
            .base
            .db()
            .list_products_feed(cursor_ts, decoded.as_ref().map(|c| c.id), limit_plus_one)
            .await?;

        let has_more = (rows.len() as i64) > limit;
        let mut rows = rows;
        if has_more {
            rows.truncate(limit as usize);
        }

        let next_cursor = if has_more {
            let last = rows.last().ok_or_else(|| anyhow::anyhow!("Unexpected empty page"))?;

            let payload = ProductCursor {
                created_at_ms: last.created_at.timestamp_millis(),
                id: last.id,
            };

            Some(encode_cursor(&payload)?)
        } else {
            None
        };

        let items = rows.into_iter().map(ProductDto::from).collect();

        Ok(Some(ProductFeedResponse { items, next_cursor }))
    }
}
