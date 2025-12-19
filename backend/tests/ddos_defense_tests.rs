#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt; // for `oneshot` method

    #[tokio::test]
    async fn test_rate_limiting() {
        let app = crate::app(); // Assuming `app` is the function that builds the Axum app

        let client_ip = "192.168.1.1";
        for _ in 0..10 {
            let response = app
                .oneshot(
                    Request::builder()
                        .uri("/health")
                        .header("X-Forwarded-For", client_ip)
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            if response.status() == StatusCode::TOO_MANY_REQUESTS {
                assert!(true, "Rate limit exceeded as expected");
                return;
            }
        }

        panic!("Rate limiting did not trigger");
    }
}