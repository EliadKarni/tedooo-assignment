use crate::cache::redis_controller::RedisCache;
use crate::db::db_controller::MySQLController;
use crate::repositories::Repos;
use crate::models::dto::HealthStatus;

#[derive(Clone)]
pub struct AppState {
    pub repos: Repos,
}

impl AppState {
    pub fn new(db: MySQLController, cache: RedisCache) -> Self {
        Self { repos: Repos::new(db, cache) }
    }
    
    pub async fn health(&self) -> Result<HealthStatus, anyhow::Error> {
        self.repos.health().await
    }
}