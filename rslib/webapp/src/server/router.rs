// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Json},
    routing::get,
    Router,
};
use serde_json::json;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health_handler))
        .route("/api/v1/info", get(info_handler))
}

async fn root_handler() -> Html<&'static str> {
    Html(
        r#"
<!DOCTYPE html>
<html>
<head>
    <title>Anki Web App</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
            max-width: 800px;
            margin: 50px auto;
            padding: 20px;
            line-height: 1.6;
        }
        h1 { color: #0a84ff; }
        code {
            background: #f5f5f5;
            padding: 2px 6px;
            border-radius: 3px;
        }
        .status { color: #28a745; }
    </style>
</head>
<body>
    <h1>🎴 Anki Web App</h1>
    <p class="status">✅ Server is running!</p>
    
    <h2>Available Endpoints</h2>
    <ul>
        <li><code>GET /</code> - This page</li>
        <li><code>GET /health</code> - Health check</li>
        <li><code>GET /api/v1/info</code> - Server info (JSON)</li>
    </ul>
    
    <h2>Next Steps</h2>
    <p>The server is running but authentication and API endpoints are not yet implemented.</p>
    <p>See <code>TASKS.md</code> for the implementation roadmap.</p>
    
    <h2>Configuration</h2>
    <p>Configure via environment variables:</p>
    <ul>
        <li><code>ANKI_WEBAPP_HOST</code> - Bind address (default: 127.0.0.1)</li>
        <li><code>ANKI_WEBAPP_PORT</code> - Port (default: 8080)</li>
        <li><code>ANKI_WEBAPP_DATA_DIR</code> - Data directory (default: ./data)</li>
        <li><code>ANKI_WEBAPP_JWT_SECRET</code> - JWT secret (⚠️ required for production)</li>
        <li><code>RUST_LOG</code> - Log level (default: info)</li>
    </ul>
</body>
</html>
        "#,
    )
}

async fn health_handler() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

async fn info_handler() -> impl IntoResponse {
    Json(json!({
        "success": true,
        "data": {
            "name": "Anki Web App",
            "version": env!("CARGO_PKG_VERSION"),
            "status": "running",
            "features": {
                "authentication": false,
                "api": false,
                "ui": false,
            },
            "message": "Server is running. API endpoints not yet implemented."
        }
    }))
}
