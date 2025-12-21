use axum::Router;
use std::net::SocketAddr;

mod routes;
mod logger;
mod controllers;
mod db;
mod models;
mod utils;
mod middlewares;
mod cache;

use models::AppState;
use db::db_controller::MySQLController;
use cache::redis_controller::RedisCache;

use logger::init_logger;
use log::info;


#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenvy::dotenv().ok();

    init_logger().expect("Failed to initialize logger");

    let state = AppState {
        tedooodb_pool: MySQLController::new().await.expect("Failed to create tedooo DB controller"),
        redis_cache: RedisCache::new().await.expect("Failed to create Redis cache"),
    };

    let app_missing_state: Router<AppState> = Router::<AppState>::new()
        .merge(routes::health::router())
        .merge(routes::sellers::router())
        .merge(routes::products::router())
        .layer(axum::middleware::from_fn(
            middlewares::log_request_middleware,
        ));

    let app = app_missing_state.with_state(state);
    let addr: SocketAddr = ([0, 0, 0, 0], 8080).into();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    info!("Server running at http://{}", addr);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}