use axum::{http::Request, middleware::Next, response::Response};
use axum::body::Body;
use log::info;

pub async fn log_request_middleware(req: Request<Body>, next: Next) -> Response {
	let method = req.method().clone();
	let path = req.uri().path().to_string();
	info!("Incoming request: {} {}", method, path);
	next.run(req).await
}
