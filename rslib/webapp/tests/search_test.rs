// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use serde_json::json;
mod common;
use common::TestContext;

#[tokio::test]
async fn test_search_and_replace() {
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

    // 2. Get notetype and deck
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

    // 3. Create two notes with searchable content
    let resp = ctx.client
        .post(format!("{}/api/v1/notes", ctx.base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "deck_id": deck_id,
            "notetype_id": notetype_id,
            "fields": ["UniqueApple", "Description one"],
            "tags": ["fruit"]
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 201);

    let resp = ctx.client
        .post(format!("{}/api/v1/notes", ctx.base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "deck_id": deck_id,
            "notetype_id": notetype_id,
            "fields": ["UniqueBanana", "Description two"],
            "tags": ["fruit"]
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 201);
    let body: serde_json::Value = resp.json().await.unwrap();
    let note_id_banana = body["note_id"].as_i64().unwrap();

    // 4. Search for notes
    let resp = ctx.client
        .post(format!("{}/api/v1/search/notes", ctx.base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({ "query": "Unique*" }))
        .send()
        .await
        .unwrap();
    
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["count"], 2);

    // 5. Search for cards
    let resp = ctx.client
        .post(format!("{}/api/v1/search/cards", ctx.base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({ "query": "UniqueBanana" }))
        .send()
        .await
        .unwrap();
    
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["count"], 1);

    // 6. Find and replace
    let resp = ctx.client
        .post(format!("{}/api/v1/search/find-replace", ctx.base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "note_ids": [note_id_banana],
            "search": "UniqueBanana",
            "replacement": "SuperBanana",
            "regex": false,
            "match_case": true
        }))
        .send()
        .await
        .unwrap();
    
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["replaced_count"], 1);

    // 7. Verify replacement
    let resp = ctx.client
        .get(format!("{}/api/v1/notes/{}", ctx.base_url, note_id_banana))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["fields"][0], "SuperBanana");
}
