// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use serde_json::json;
mod common;
use common::TestContext;
use reqwest::multipart;

#[tokio::test]
async fn test_media_flow() {
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

    let body: serde_json::Value = resp.json().await.unwrap();
    let token = body["data"]["token"].as_str().unwrap();

    // 2. Upload a file
    let form = multipart::Form::new()
        .part("file", multipart::Part::bytes(vec![0xFF, 0xD8, 0xFF, 0xEE]).file_name("test.jpg"));

    let resp = ctx.client
        .post(format!("{}/api/v1/media", ctx.base_url))
        .header("Authorization", format!("Bearer {}", token))
        .multipart(form)
        .send()
        .await
        .unwrap();
    
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body["success"].as_bool().unwrap());
    let filename = body["filename"].as_str().unwrap().to_string();

    // 3. Get the file
    let resp = ctx.client
        .get(format!("{}/api/v1/media/{}", ctx.base_url, filename))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.headers()["content-type"], "image/jpeg");
    let data = resp.bytes().await.unwrap();
    assert_eq!(data.as_ref(), &[0xFF, 0xD8, 0xFF, 0xEE]);

    // 4. Check media
    let resp = ctx.client
        .get(format!("{}/api/v1/media/check", ctx.base_url))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body["unused"].as_array().unwrap().iter().any(|v| v.as_str().unwrap() == filename));

    // 5. Delete the file
    let resp = ctx.client
        .delete(format!("{}/api/v1/media", ctx.base_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "filenames": [filename]
        }))
        .send()
        .await
        .unwrap();
    
    assert_eq!(resp.status(), 200);

    // 6. Verify deletion
    let resp = ctx.client
        .get(format!("{}/api/v1/media/{}", ctx.base_url, filename))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    
    assert_eq!(resp.status(), 404);
}
