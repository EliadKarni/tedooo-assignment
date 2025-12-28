use backend::{create_app_state, create_router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub struct TestApp {
    pub address: String,
}

pub async fn spawn_app() -> TestApp {
    #[cfg(debug_assertions)]
    dotenvy::dotenv().ok();

    let state = create_app_state().await.expect("Failed to create app state");
    let app = create_router(state);

    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    tokio::spawn(async move {
        axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .unwrap();
    });

    TestApp { address }
}
