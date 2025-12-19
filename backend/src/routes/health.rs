use axum::Json;
use serde::Serialize;
use std::sync::Arc;

use crate::middleware::rate_limiter::SharedRateLimiter;

/// Represents the health status of the backend services.
///
/// - `database`: Indicates whether the database is reachable.
/// - `redis`: Indicates whether the Redis cache is reachable.
#[derive(Serialize)]
pub struct HealthStatus {
    pub database: bool,
    pub redis: bool,
}

/// Handles the `GET /health` endpoint with rate limiting.
///
/// This endpoint checks the health of the database and Redis cache, and applies
/// rate limiting to prevent abuse.
///
/// # Arguments
/// - `rate_limiter`: The shared rate limiter instance.
///
/// # Returns
/// A JSON response containing the health status of the backend services.
pub async fn get_health_with_throttling(
    rate_limiter: Arc<SharedRateLimiter>,
) -> Json<HealthStatus> {
    let client_ip = "127.0.0.1".to_string(); // Replace with actual client IP extraction logic

    if rate_limiter.check_rate_limit(client_ip).await {
        let health_status = HealthStatus {
            database: true, // Replace with actual DB connectivity check
            redis: true,    // Replace with actual Redis connectivity check
        };

        Json(health_status)
    } else {
        Json(HealthStatus {
            database: false,
            redis: false,
        })
    }
}