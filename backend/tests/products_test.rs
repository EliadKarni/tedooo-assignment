mod common;
use serde_json::Value;

#[tokio::test]
async fn get_products_feed_works() {
    let app = common::spawn_app().await;
    let client = reqwest::Client::new();

    // Generate data first to ensure we have something
    let _ = client
        .put(&format!("{}/generate-products", &app.address))
        .send()
        .await;

    let response = client
        .get(&format!("{}/products", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    
    let body: Value = response.json().await.expect("Failed to parse JSON");
    assert!(body.get("items").is_some());
    assert!(body.get("items").unwrap().is_array());
}

#[tokio::test]
async fn get_product_by_id_works() {
    let app = common::spawn_app().await;
    let client = reqwest::Client::new();

    // Generate data first
    let _ = client
        .put(&format!("{}/generate-products", &app.address))
        .send()
        .await;

    // First get the feed to find a valid ID
    let feed_response = client
        .get(&format!("{}/products", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");
    
    let feed_body: Value = feed_response.json().await.expect("Failed to parse JSON");
    let items = feed_body.get("items").and_then(|v| v.as_array());

    if let Some(items) = items {
        if let Some(first_item) = items.first() {
            let id = first_item.get("id").and_then(|v| v.as_i64()).unwrap();
            
            let product_response = client
                .get(&format!("{}/products/{}", &app.address, id))
                .send()
                .await
                .expect("Failed to execute request.");
            
            assert!(product_response.status().is_success());
            let product_body: Value = product_response.json().await.expect("Failed to parse JSON");
            assert_eq!(product_body.get("id").and_then(|v| v.as_i64()), Some(id));
        }
    }
}
