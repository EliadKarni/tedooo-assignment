#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt; // for `oneshot` method

    #[tokio::test]
    async fn test_get_products() {
        let app = crate::app(); // Assuming `app` is the function that builds the Axum app

        let response = app
            .oneshot(Request::builder().uri("/products").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert!(std::str::from_utf8(&body).unwrap().contains("Sample Product"));
    }

    #[tokio::test]
    async fn test_get_product_details() {
        let app = crate::app();

        let response = app
            .oneshot(Request::builder().uri("/products/1").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert!(std::str::from_utf8(&body).unwrap().contains("Sample Seller"));
    }

    #[tokio::test]
    async fn test_health_check() {
        let app = crate::app();

        let response = app
            .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert!(std::str::from_utf8(&body).unwrap().contains("database"));
    }
}