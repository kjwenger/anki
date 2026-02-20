// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use serde_json::json;
mod common;
use common::TestContext;

#[tokio::test]
async fn test_auth_flow() {
    let ctx = TestContext::new().await;

    // 1. Register
    let resp = ctx.client
        .post(format!("{}/api/v1/auth/register", ctx.base_url))
        .json(&json!({
            "username": "testuser",
            "password": "password123"
        }))
        .send()
        .await
        .expect("Failed to send register request");

    assert_eq!(resp.status(), 201);
    let body: serde_json::Value = resp.json().await.expect("Failed to parse register response");
    assert!(body["success"].as_bool().unwrap());

    // 2. Login
    let resp = ctx.client
        .post(format!("{}/api/v1/auth/login", ctx.base_url))
        .json(&json!({
            "username": "testuser",
            "password": "password123"
        }))
        .send()
        .await
        .expect("Failed to send login request");

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.expect("Failed to parse login response");
    assert!(body["success"].as_bool().unwrap());
    let token = body["data"]["token"].as_str().expect("Token missing in response");

    // 3. Access protected route
    let resp = ctx.client
        .get(format!("{}/api/v1/auth/me", ctx.base_url))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("Failed to send me request");

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.expect("Failed to parse me response");
    assert_eq!(body["username"], "testuser");

    // 4. Logout
    let resp = ctx.client
        .post(format!("{}/api/v1/auth/logout", ctx.base_url))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("Failed to send logout request");

    assert_eq!(resp.status(), 200);

    // 5. Verify token is invalidated (or at least user is logged out)
    let resp = ctx.client
        .get(format!("{}/api/v1/auth/me", ctx.base_url))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("Failed to send me request after logout");

    assert_eq!(resp.status(), 401);
}

#[tokio::test]
async fn test_invalid_login() {
    let ctx = TestContext::new().await;

    // Attempt login without registration
    let resp = ctx.client
        .post(format!("{}/api/v1/auth/login", ctx.base_url))
        .json(&json!({
            "username": "nonexistent",
            "password": "wrongpassword"
        }))
        .send()
        .await
        .expect("Failed to send login request");

    assert_eq!(resp.status(), 401);
}
