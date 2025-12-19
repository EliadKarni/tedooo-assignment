use axum::{http::Request, middleware::Next, response::Response};
use crate::auth::jwt::validate_jwt;

pub async fn jwt_auth<B>(req: Request<B>, next: Next<B>) -> Result<Response, &'static str> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                if validate_jwt(token).is_ok() {
                    return Ok(next.run(req).await);
                }
            }
        }
    }

    Err("Unauthorized")
}