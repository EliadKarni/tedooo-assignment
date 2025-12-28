use std::net::SocketAddr;
use log::info;
use backend::{create_app_state, create_router, logger::init_logger};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(debug_assertions)]
    dotenvy::dotenv().ok();
    init_logger().expect("Failed to initialize logger");

    let state = create_app_state().await.expect("Failed to create app state");
    let app = create_router(state);

    let addr: SocketAddr = ([0, 0, 0, 0], 8080).into();
    let listener = tokio::net::TcpListener::bind(addr).await?;

    info!("Server running at http://{}", addr);
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    Ok(())
}