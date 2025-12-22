use crate::db::db_controller::MySQLController;
use crate::cache::redis_controller::RedisCache;

#[derive(Clone)]
pub struct AppState {
    pub tedooodb_pool: MySQLController,
    pub redis_cache: RedisCache,
}