// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use serde_json::json;
mod common;
use common::TestContext;

#[tokio::test]
async fn test_scheduler_flow() {
    let ctx = TestContext::new().await;

    // 1. Setup Auth
    let resp = ctx.client
        .post(format!("{}/api/v1/auth/register", ctx.base_url))
        .json(&json!({
            "username": "testuser",
            "password": "password123"
        }))
        .send()
        .await
        .expect("Failed to register");
    assert_eq!(resp.status(), 201);

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

    // 2. Get default deck ID
    let resp = ctx.client
        .get(format!("{}/api/v1/decks", ctx.base_url))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    let body: serde_json::Value = resp.json().await.unwrap();
    let deck_id = body["decks"][0]["id"].as_i64().unwrap();

    // 3. Create a note (which generates a card)
    let resp = ctx.client
        .get(format!("{}/api/v1/notetypes", ctx.base_url))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    let body: serde_json::Value = resp.json().await.unwrap();
    let notetype_id = body["notetypes"][0]["id"].as_i64().unwrap();

    let resp = ctx.client
        .post(format!("{}/api/v1/notes", ctx.base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "deck_id": deck_id,
            "notetype_id": notetype_id,
            "fields": ["Question", "Answer"],
            "tags": []
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 201);

    // 4. Get next card
    let resp = ctx.client
        .get(format!("{}/api/v1/scheduler/decks/{}/next", ctx.base_url, deck_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(!body["finished"].as_bool().unwrap());
    let card_id = body["card"]["card_id"].as_i64().unwrap();
    assert_eq!(body["card"]["counts"]["new"], 1);

    // 5. Get next states (intervals)
    let resp = ctx.client
        .get(format!("{}/api/v1/scheduler/decks/{}/cards/{}/next-states", ctx.base_url, deck_id, card_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body["again"].is_string());
    assert!(body["good"].is_string());

    // 6. Answer the card
    let resp = ctx.client
        .post(format!("{}/api/v1/scheduler/decks/{}/cards/{}/answer", ctx.base_url, deck_id, card_id))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "rating": 2, // Good
            "milliseconds_taken": 5000
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body["success"].as_bool().unwrap());

    // 7. Verify counts updated
    let resp = ctx.client
        .get(format!("{}/api/v1/scheduler/decks/{}/counts", ctx.base_url, deck_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    
    let body: serde_json::Value = resp.json().await.unwrap();
    // New should be 0, Learning should be 1
    assert_eq!(body["new"], 0);
    assert_eq!(body["learning"], 1);
}
