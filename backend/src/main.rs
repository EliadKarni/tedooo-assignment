use axum::Router;
use std::net::SocketAddr;

mod routes;
mod controllers;
mod db;
mod models;
mod utils;
mod middlewares;
mod cache;
mod logger;
mod repositories;
mod state;

use db::db_controller::MySQLController;
use cache::redis_controller::RedisCache;
use log::info;

use crate::logger::init_logger;
use crate::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(debug_assertions)]
    dotenvy::dotenv().ok();
    init_logger().expect("Failed to initialize logger");

    let state = AppState::new(MySQLController::new().expect("Failed to create DB controller"),
        RedisCache::new());

    let app_missing_state: Router<AppState> = Router::<AppState>::new()
        .merge(routes::health::router())
        .merge(routes::sellers::router())
        .merge(routes::products::router())
        .layer(axum::middleware::from_fn(
            middlewares::log_request_middleware,
        ));

    let app = app_missing_state.with_state(state);

    let addr: SocketAddr = ([0, 0, 0, 0], 8080).into();
    let listener = tokio::net::TcpListener::bind(addr).await?;

    info!("Server running at http://{}", addr);
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    Ok(())
}