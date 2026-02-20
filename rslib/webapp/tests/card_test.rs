// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use serde_json::json;
mod common;
use common::TestContext;

#[tokio::test]
async fn test_card_crud() {
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
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    let token = body["data"]["token"].as_str().unwrap();

    // 2. Create a note and get its cards
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

    let resp = ctx.client
        .post(format!("{}/api/v1/notes", ctx.base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "deck_id": deck_id,
            "notetype_id": notetype_id,
            "fields": ["Front content", "Back content"],
            "tags": ["test"]
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

    // 3. Get the card
    let resp = ctx.client
        .get(format!("{}/api/v1/cards/{}", ctx.base_url, card_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["id"], card_id);
    assert_eq!(body["note_id"], note_id);
    assert_eq!(body["deck_id"], deck_id);

    // 4. Update the card
    let resp = ctx.client
        .put(format!("{}/api/v1/cards/{}", ctx.base_url, card_id))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "due": 1000,
            "flags": 1
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    // Verify update
    let resp = ctx.client
        .get(format!("{}/api/v1/cards/{}", ctx.base_url, card_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["due"], 1000);
    assert_eq!(body["flags"], 1);

    // 5. Card status actions
    let actions = vec!["flag", "suspend", "unsuspend", "bury"];
    for action in actions {
        let url = format!("{}/api/v1/cards/{}/{}", ctx.base_url, card_id, action);
        let resp = if action == "flag" {
            ctx.client.post(url)
                .header("Authorization", format!("Bearer {}", token))
                .json(&json!({ "flag": 2 }))
                .send()
                .await
                .unwrap()
        } else {
            ctx.client.post(url)
                .header("Authorization", format!("Bearer {}", token))
                .send()
                .await
                .unwrap()
        };
        assert_eq!(resp.status(), 200);
    }

    // 6. Batch get
    let resp = ctx.client
        .post(format!("{}/api/v1/cards/batch", ctx.base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({ "card_ids": [card_id] }))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["cards"][0]["id"], card_id);

    // 7. Batch update
    let resp = ctx.client
        .post(format!("{}/api/v1/cards/batch-update", ctx.base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "updates": [{
                "card_id": card_id,
                "due": 2000,
                "flags": 3
            }]
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    // 8. Delete the card
    let resp = ctx.client
        .delete(format!("{}/api/v1/cards/{}", ctx.base_url, card_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    
    if resp.status() != 200 {
        let status = resp.status();
        let body: serde_json::Value = resp.json().await.unwrap();
        panic!("Delete failed with status {}: {:?}", status, body);
    }
    assert_eq!(resp.status(), 200);

    // Verify deletion
    let resp = ctx.client
        .get(format!("{}/api/v1/cards/{}", ctx.base_url, card_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();

    if resp.status() != 404 {
        let status = resp.status();
        let body: serde_json::Value = resp.json().await.unwrap();
        panic!("Verification failed with status {}: {:?}", status, body);
    }
    assert_eq!(resp.status(), 404);
}
