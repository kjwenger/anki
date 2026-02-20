// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use serde_json::json;
mod common;
use common::TestContext;

#[tokio::test]
async fn test_collection_flow() {
    let ctx = TestContext::new().await;

    // 1. Register and Login
    ctx.client
        .post(format!("{}/api/v1/auth/register", ctx.base_url))
        .json(&json!({
            "username": "testuser",
            "password": "password123"
        }))
        .send()
        .await
        .expect("Failed to register");

    let resp = ctx.client
        .post(format!("{}/api/v1/auth/login", ctx.base_url))
        .json(&json!({
            "username": "testuser",
            "password": "password123"
        }))
        .send()
        .await
        .expect("Failed to login");

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.expect("Failed to parse login response");
    let token = body["data"]["token"].as_str().unwrap();

    // 2. Get Collection Info (this also triggers backend initialization)
    let resp = ctx.client
        .get(format!("{}/api/v1/collection/info", ctx.base_url))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("Failed to get collection info");

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["username"], "testuser");
    assert!(body["backend_active"].as_bool().unwrap());

    // 3. List Collections
    let resp = ctx.client
        .get(format!("{}/api/v1/collections", ctx.base_url))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("Failed to list collections");

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body["collections"].is_array());
    assert_eq!(body["collections"][0]["name"], "testuser's Collection");

    // 4. Create Collection (no-op in current implementation but should return 200)
    let resp = ctx.client
        .post(format!("{}/api/v1/collections", ctx.base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "name": "New Collection"
        }))
        .send()
        .await
        .expect("Failed to create collection");

    assert_eq!(resp.status(), 201);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body["path"].as_str().unwrap().contains("New_Collection"));

    // 5. Close Collection
    let resp = ctx.client
        .post(format!("{}/api/v1/collection/close", ctx.base_url))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("Failed to close collection");

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body["success"].as_bool().unwrap());
}
