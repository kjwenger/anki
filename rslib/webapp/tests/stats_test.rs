// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use serde_json::json;
mod common;
use common::TestContext;

#[tokio::test]
async fn test_stats() {
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

    // 2. Get available notetype and deck
    let resp = ctx.client
        .get(format!("{}/api/v1/notetypes", ctx.base_url))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    let body: serde_json::Value = resp.json().await.unwrap();
    let notetype_id = body["notetypes"][0]["id"].as_i64().unwrap();

    let resp = ctx.client
        .get(format!("{}/api/v1/decks", ctx.base_url))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    let body: serde_json::Value = resp.json().await.unwrap();
    let deck_id = body["decks"][0]["id"].as_i64().unwrap();

    // 3. Create a note
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
    let body: serde_json::Value = resp.json().await.unwrap();
    let note_id = body["note_id"].as_i64().unwrap();

    let resp = ctx.client
        .get(format!("{}/api/v1/notes/{}/cards", ctx.base_url, note_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    let body: serde_json::Value = resp.json().await.unwrap();
    let card_id = body["card_ids"][0].as_i64().unwrap();

    // 4. Get collection stats
    let resp = ctx.client
        .get(format!("{}/api/v1/stats/collection", ctx.base_url))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["total_cards"], 1);
    assert_eq!(body["new_cards"], 1);

    // 5. Get today stats
    let resp = ctx.client
        .get(format!("{}/api/v1/stats/today", ctx.base_url))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("answer_count").is_some());

    // 6. Get card stats
    let resp = ctx.client
        .get(format!("{}/api/v1/stats/card/{}", ctx.base_url, card_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["card_id"], card_id);
    assert_eq!(body["note_id"], note_id);
}
