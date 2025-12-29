use std::{sync::Arc, time::Duration};

use crate::db::db_controller::DbError;
use crate::models::row::seller::SellerRow;

use super::BaseRepository;

#[derive(Clone)]
pub struct SellerRepository {
    base: Arc<BaseRepository>, // private field with private type - OK
}

impl SellerRepository {
    /// Create a new SellerRepository.
    pub(super) fn new(base: Arc<BaseRepository>) -> Self {
        Self { base }
    }

    fn get_seller_key(id: i64) -> String {
        format!("sellers:by_id:{}", id)
    }

    pub async fn get_seller(&self, id: i64) -> anyhow::Result<Option<SellerRow>> {
        let key = Self::get_seller_key(id);

        // Cache stores Option<SellerRow> (Some / None)
        match self.base.cache().get::<Option<SellerRow>>(&key).await {
            Ok(Some(v)) => return Ok(v), // v is Option<SellerRow>
            Ok(None) => { /* cache miss */ }
            Err(_) => { /* cache failure treated as miss */ }
        }

        let v = self.base.db().get_seller_by_id(id).await?;

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

    pub async fn generate_sellers(&self, count: i64) -> Result<bool, DbError> {
        match self.base.db().generate_sellers(count).await{
            Ok(result) => Ok(result),
            Err(e) => return Err(e),
        }
    }
}

