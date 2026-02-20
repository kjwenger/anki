// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use serde_json::json;
mod common;
use common::TestContext;

#[tokio::test]
async fn test_deck_crud() {
    let ctx = TestContext::new().await;

    // 1. Setup Auth
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
    let body: serde_json::Value = resp.json().await.unwrap();
    let token = body["data"]["token"].as_str().unwrap();

    // 2. List initial decks (Default deck should exist)
    let resp = ctx.client
        .get(format!("{}/api/v1/decks", ctx.base_url))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body["decks"].is_array());
    assert_eq!(body["decks"][0]["name"], "Default");
    let _default_id = body["decks"][0]["id"].as_i64().unwrap();

    // 3. Create a new deck
    let resp = ctx.client
        .post(format!("{}/api/v1/decks", ctx.base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "name": "New Test Deck"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 201);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body["success"].as_bool().unwrap());
    let new_deck_id = body["id"].as_i64().expect("ID missing in response");

    // 4. Get the new deck by ID
    let resp = ctx.client
        .get(format!("{}/api/v1/decks/{}", ctx.base_url, new_deck_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["id"], new_deck_id);
    assert_eq!(body["name"], "New Test Deck");

    // 5. Update the deck name
    let resp = ctx.client
        .put(format!("{}/api/v1/decks/{}", ctx.base_url, new_deck_id))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "name": "Updated Deck Name"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body["success"].as_bool().unwrap());

    // Verify rename
    let resp = ctx.client
        .get(format!("{}/api/v1/decks/{}", ctx.base_url, new_deck_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();

    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["name"], "Updated Deck Name");

    // 6. Delete the deck
    let resp = ctx.client
        .delete(format!("{}/api/v1/decks/{}", ctx.base_url, new_deck_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body["success"].as_bool().unwrap());

    // 7. Verify deletion
    let resp = ctx.client
        .get(format!("{}/api/v1/decks/{}", ctx.base_url, new_deck_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 404);
}
