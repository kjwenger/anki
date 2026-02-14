// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use axum::{
    http::StatusCode,
    middleware,
    response::{Html, IntoResponse, Json},
    routing::{delete, get, post},
    Router,
};
use serde_json::json;

use crate::auth::{require_auth, AuthState};
use crate::routes::{
    close_collection, create_deck, delete_deck, get_collection_info, get_deck, get_deck_tree,
    login, logout, me, register, AuthRouteState,
};
use crate::WebAppConfig;

pub fn create_router(config: &WebAppConfig, auth_state: AuthState) -> Router<()> {
    // Routes that require authentication
    let protected_routes = Router::new()
        .route("/api/v1/auth/logout", post(logout))
        .route("/api/v1/auth/me", get(me))
        .route("/api/v1/collection/info", get(get_collection_info))
        .route("/api/v1/collection/close", post(close_collection))
        .route("/api/v1/decks", get(get_deck_tree))
        .route("/api/v1/decks", post(create_deck))
        .route("/api/v1/decks/{id}", get(get_deck))
        .route("/api/v1/decks/{id}", delete(delete_deck))
        .layer(middleware::from_fn_with_state(
            auth_state.clone(),
            require_auth,
        ));

    // Public auth routes  
    let auth_route_state = AuthRouteState {
        database: auth_state.database.clone(),
        jwt_manager: auth_state.jwt_manager.clone(),
        backend_manager: auth_state.backend_manager.clone(),
        session_timeout_hours: config.session_timeout_hours as i64,
    };

    let public_routes = Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health_handler))
        .route("/api/v1/info", get(info_handler))
        .route("/api/v1/auth/register", post(register))
        .route("/api/v1/auth/login", post(login));

    // Combine routes
    public_routes
        .merge(protected_routes)
        .with_state(auth_route_state)
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
    <h3>Public Endpoints</h3>
    <ul>
        <li><code>GET /</code> - This page</li>
        <li><code>GET /health</code> - Health check</li>
        <li><code>GET /api/v1/info</code> - Server info (JSON)</li>
        <li><code>POST /api/v1/auth/register</code> - Register new user</li>
        <li><code>POST /api/v1/auth/login</code> - Login user</li>
    </ul>
    
    <h3>Protected Endpoints (Require Authentication)</h3>
    <ul>
        <li><code>GET /api/v1/auth/me</code> - Get current user info</li>
        <li><code>POST /api/v1/auth/logout</code> - Logout user</li>
        <li><code>GET /api/v1/collection/info</code> - Get collection info</li>
        <li><code>POST /api/v1/collection/close</code> - Close collection</li>
        <li><code>GET /api/v1/decks</code> - Get deck tree</li>
        <li><code>POST /api/v1/decks</code> - Create deck</li>
        <li><code>GET /api/v1/decks/{id}</code> - Get deck by ID</li>
        <li><code>DELETE /api/v1/decks/{id}</code> - Delete deck</li>
    </ul>
    
    <h2>Status</h2>
    <p>✅ Authentication system is now active!</p>
    <p>Features available:</p>
    <ul>
        <li>✅ User registration with password hashing (Argon2)</li>
        <li>✅ JWT-based authentication</li>
        <li>✅ Session management with expiration</li>
        <li>✅ Protected routes with middleware</li>
    </ul>
    
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
                "authentication": true,
                "api": false,
                "ui": false,
            },
            "message": "Server is running. Authentication enabled."
        }
    }))
}
