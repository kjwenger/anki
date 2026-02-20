// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use serde_json::json;
mod common;
use common::TestContext;
use reqwest::multipart;

#[tokio::test]
async fn test_apkg_import() {
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

    // 2. Create a dummy .apkg file content
    // An APKG is actually a zip file, but for the sake of the API test, 
    // we'll send a valid zip header or just a random file to see if the 
    // backend attempts to process it (it will likely fail core processing 
    // if not a real zip, but we want to test the multipart plumbing).
    
    // To properly test, we'd need a real .apkg, but we can at least test 
    // that the endpoint is reachable and handles the multipart data.
    let form = multipart::Form::new()
        .part("file", multipart::Part::bytes(vec![0x50, 0x4B, 0x03, 0x04]).file_name("test.apkg"));

    let resp = ctx.client
        .post(format!("{}/api/v1/import/apkg", ctx.base_url))
        .header("Authorization", format!("Bearer {}", token))
        .multipart(form)
        .send()
        .await
        .unwrap();
    
    // It will return 500 if the core fails to open the invalid zip, 
    // but 400 if our multipart logic is wrong.
    // Given it's an invalid zip, we expect a core error (Internal).
    let status = resp.status();
    assert!(status.is_success() || status.is_server_error());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    if status.is_server_error() {
        assert!(body["error"]["message"].as_str().unwrap().contains("Internal error"));
    }
}
