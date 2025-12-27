use std::sync::Arc;

use crate::cache::redis_controller::RedisCache;
use crate::db::db_controller::MySQLController;

pub mod product_repository;
pub mod seller_repository;

pub use product_repository::ProductRepository;
pub use seller_repository::SellerRepository;

pub use crate::models::dto::HealthStatus;

/// A container for all repositories used by the app.
#[derive(Clone)]
pub struct Repos {
    base: Arc<BaseRepository>,
    pub products: ProductRepository,
    pub sellers: SellerRepository,
}

/// Build all repositories from shared DB + cache handles.
///
/// Note: BaseRepository is private and created only once, then shared via Arc.
impl Repos {
    pub fn new(db: MySQLController, cache: RedisCache) -> Self {
        let base = Arc::new(BaseRepository::new(db, cache, 300));

        Self {
            base: base.clone(),
            products: ProductRepository::new(base.clone()),
            sellers: SellerRepository::new(base.clone()),
        }
    }

    pub async fn health(&self) -> HealthStatus {
        // assuming `self.base: Arc<BaseRepository>` exists inside Repos
        // or you can keep a separate `base` field just for health.
        let db_ok = self.base.db().is_db_available().await;
        let redis_ok = self.base.cache().is_available().await;

        HealthStatus { database: db_ok, redis: redis_ok }
    }
}

/// Internal shared repository base (private).
struct BaseRepository {
    db: MySQLController,
    cache: RedisCache,
    ttl_seconds: usize,
}

impl BaseRepository {
    /// Create a new BaseRepository.
    fn new(db: MySQLController, cache: RedisCache, ttl_seconds: usize) -> Self {
        Self { db, cache, ttl_seconds }
    }

    // Keep your get_or_fetch/get_or_fetch_default_ttl methods here (they can be `fn`/`pub(crate)`).
    // Also keep internal accessors if needed:
    fn db(&self) -> &MySQLController { &self.db }
    fn cache(&self) -> &RedisCache { &self.cache }
    fn ttl_seconds(&self) -> usize { self.ttl_seconds }
}
