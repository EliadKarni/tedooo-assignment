use crate::db::db_controller::MySQLController;
use crate::cache::redis_controller::RedisCache;
use crate::models::AppState;

pub struct SellersRepository {
    pub tedooodb_pool: MySQLController,
    pub redis_cache: RedisCache,
}

impl SellersRepository {
    pub fn new(app_state: AppState) -> Self {
        Self {
            tedooodb_pool: app_state.tedooodb_pool,
            redis_cache: app_state.redis_cache,
        }

    }
    /*
    async fn get_by_id(&self, id: i64) -> Result<Option<Device>, RepositoryError> {
        let rec = sqlx::query_as!(Device, "SELECT id, name, status FROM devices WHERE id = ?", id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(rec)
    }
    async fn create(&self, entity: Device) -> Result<Device, RepositoryError> {
        entity.validate().map_err(|e| RepositoryError::Validation(format!("{:?}", e)))?;
        let rec = sqlx::query_as!(Device, "INSERT INTO devices (name, status) VALUES (?, ?) RETURNING id, name, status", entity.name, entity.status)
            .fetch_one(&self.pool)
            .await?;
        Ok(rec)
    }
    async fn update(&self, entity: Device) -> Result<Device, RepositoryError> {
        entity.validate().map_err(|e| RepositoryError::Validation(format!("{:?}", e)))?;
        let rec = sqlx::query_as!(Device, "UPDATE devices SET name = ?, status = ? WHERE id = ? RETURNING id, name, status", entity.name, entity.status, entity.id)
            .fetch_one(&self.pool)
            .await?;
        Ok(rec)
    }
    async fn delete(&self, id: i64) -> Result<(), RepositoryError> {
        sqlx::query!("DELETE FROM devices WHERE id = ?", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }*/
}
