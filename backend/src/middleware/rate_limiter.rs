use axum::{http::StatusCode, middleware::Next, response::IntoResponse, Request};
use governor::{Quota, RateLimiter};
use nonzero_ext::nonzero;
use std::num::NonZeroU32;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower::ServiceBuilder;

/// A shared rate limiter for managing request limits.
///
/// This struct uses the `governor` crate to enforce rate limits on incoming requests.
pub struct SharedRateLimiter {
    limiter: Arc<Mutex<RateLimiter<String, governor::state::keyed::DefaultKeyedStateStore>>>,
}

impl SharedRateLimiter {
    /// Creates a new SharedRateLimiter instance.
    ///
    /// # Arguments
    /// - `requests_per_second`: The maximum number of requests allowed per second.
    pub fn new(requests_per_second: NonZeroU32) -> Self {
        let limiter = RateLimiter::keyed(Quota::per_second(requests_per_second));
        SharedRateLimiter {
            limiter: Arc::new(Mutex::new(limiter)),
        }
    }

    /// Checks if a request from a specific client IP is within the rate limit.
    ///
    /// # Arguments
    /// - `client_ip`: The IP address of the client.
    ///
    /// # Returns
    /// `true` if the request is allowed, `false` otherwise.
    pub async fn check_rate_limit(&self, client_ip: String) -> bool {
        let mut limiter = self.limiter.lock().await;
        limiter.check_key(&client_ip).is_ok()
    }
}

pub async fn rate_limit<B>(
    req: Request<B>,
    next: Next<B>,
    rate_limiter: Arc<SharedRateLimiter>,
) -> impl IntoResponse {
    let client_ip = req
        .headers()
        .get("X-Forwarded-For")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    if rate_limiter.check_rate_limit(client_ip).await {
        next.run(req).await
    } else {
        (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded").into_response()
    }
}

pub fn rate_limiter_layer(requests_per_second: NonZeroU32) -> ServiceBuilder {
    let rate_limiter = Arc::new(SharedRateLimiter::new(requests_per_second));
    ServiceBuilder::new().layer_fn(move |inner| {
        tower::layer::layer_fn(move |svc| {
            tower::service_fn(move |req| rate_limit(req, inner.clone(), rate_limiter.clone()))
        })
    })
}