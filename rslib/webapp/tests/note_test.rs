// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use serde_json::json;
mod common;
use common::TestContext;

#[tokio::test]
async fn test_note_crud() {
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
        .unwrap();

    let resp = ctx.client
        .post(format!("{}/api/v1/auth/login", ctx.base_url))
        .json(&json!({
            "username": "testuser",
            "password": "password123"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    let token = body["data"]["token"].as_str().unwrap();

    // 2. Get available notetypes and default deck
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

    // 3. Check fields (should be NORMAL/0)
    let resp = ctx.client
        .post(format!("{}/api/v1/notes/check-fields", ctx.base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "notetype_id": notetype_id,
            "fields": ["Front content", "Back content"]
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["state"], 0);

    // 4. Create a note
    let resp = ctx.client
        .post(format!("{}/api/v1/notes", ctx.base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "deck_id": deck_id,
            "notetype_id": notetype_id,
            "fields": ["Front content", "Back content"],
            "tags": ["test", "integration"]
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 201);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body["success"].as_bool().unwrap());
    let note_id = body["note_id"].as_i64().unwrap();

    // 5. Get the note
    let resp = ctx.client
        .get(format!("{}/api/v1/notes/{}", ctx.base_url, note_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["id"], note_id);
    assert_eq!(body["fields"][0], "Front content");
    assert!(body["tags"].as_array().unwrap().contains(&json!("test")));

    // 6. Update the note
    let resp = ctx.client
        .put(format!("{}/api/v1/notes/{}", ctx.base_url, note_id))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "fields": ["Updated front", "Updated back"],
            "tags": ["test", "updated"]
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    // Verify update
    let resp = ctx.client
        .get(format!("{}/api/v1/notes/{}", ctx.base_url, note_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["fields"][0], "Updated front");
    assert!(body["tags"].as_array().unwrap().contains(&json!("updated")));

    // 7. Get cards for the note
    let resp = ctx.client
        .get(format!("{}/api/v1/notes/{}/cards", ctx.base_url, note_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body["card_ids"].is_array());
    assert!(!body["card_ids"].as_array().unwrap().is_empty());

    // 8. Delete the note
    let resp = ctx.client
        .delete(format!("{}/api/v1/notes/{}", ctx.base_url, note_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    // Verify deletion
    let resp = ctx.client
        .get(format!("{}/api/v1/notes/{}", ctx.base_url, note_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 404);
}
