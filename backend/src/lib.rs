pub mod routes;
pub mod controllers;
pub mod db;
pub mod models;
pub mod utils;
pub mod middlewares;
pub mod cache;
pub mod logger;
pub mod repositories;
pub mod state;

use axum::Router;
use db::db_controller::MySQLController;
use cache::redis_controller::RedisCache;
use state::AppState;

pub async fn create_app_state() -> anyhow::Result<AppState> {
    let db = MySQLController::new()?;
    let cache = RedisCache::new();
    Ok(AppState::new(db, cache))
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .merge(routes::health::router())
        .merge(routes::sellers::router())
        .merge(routes::products::router())
        .layer(axum::middleware::from_fn(
            middlewares::log_request_middleware,
        ))
        .with_state(state)
}
