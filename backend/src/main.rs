use axum::{
    extract::Extension,
    middleware,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

mod auth;
mod routes; // Import routes module

use auth::jwt::{generate_jwt, validate_jwt};

// Shared state for the app
struct AppState {
    // Add shared state here (e.g., database pool, Redis client)
}

async fn login_handler() -> String {
    // Example user ID
    let user_id = "user123";
    generate_jwt(user_id)
}

async fn protected_handler() -> &'static str {
    "This is a protected route."
}

#[tokio::main]
async fn main() {
    // Initialize shared state
    let state = Arc::new(Mutex::new(AppState {}));

    // Initialize the router with the required endpoints
    let app = Router::new()
        .route("/products", get(routes::products::get_products)) // GET /products
        .route("/products/:id", get(routes::products::get_product_by_id)) // GET /products/{id}
        .route("/health", get(routes::health::get_health)) // GET /health
        .route("/login", post(login_handler))
        .route("/protected", post(protected_handler).route_layer(middleware::from_fn(jwt_auth)))
        .layer(Extension(state)); // Add shared state as an extension

    // Define the address to bind the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    // Start the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
