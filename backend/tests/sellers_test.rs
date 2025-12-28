mod common;
use serde_json::Value;

#[tokio::test]
async fn get_seller_by_id_works() {
    let app = common::spawn_app().await;
    let client = reqwest::Client::new();

    // Generate data first
    let _ = client
        .put(&format!("{}/generate-sellers", &app.address))
        .send()
        .await;
    let _ = client
        .put(&format!("{}/generate-products", &app.address))
        .send()
        .await;

    // Get a product to find a valid seller ID
    let feed_response = client
        .get(&format!("{}/products", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");
        
    let feed_body: Value = feed_response.json().await.expect("Failed to parse JSON");
    let items = feed_body.get("items").and_then(|v| v.as_array());

    if let Some(items) = items {
        if let Some(first_item) = items.first() {
            // Assuming product has seller info
            if let Some(seller) = first_item.get("seller") {
                 let seller_id = seller.get("id").and_then(|v| v.as_i64()).unwrap();
                 
                 let seller_response = client
                    .get(&format!("{}/seller/{}", &app.address, seller_id))
                    .send()
                    .await
                    .expect("Failed to execute request.");
                    
                 assert!(seller_response.status().is_success());
                 let seller_body: Value = seller_response.json().await.expect("Failed to parse JSON");
                 assert_eq!(seller_body.get("id").and_then(|v| v.as_i64()), Some(seller_id));
            }
        }
    }
}
