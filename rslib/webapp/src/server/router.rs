// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use axum::http::StatusCode;
use axum::middleware;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Json;
use axum::routing::delete;
use axum::routing::get;
use axum::routing::post;
use axum::routing::put;
use axum::Router;
use serde_json::json;

use crate::auth::require_auth;
use crate::auth::AuthState;
use crate::openapi;
use crate::routes::batch_get_cards;
use crate::routes::batch_update_cards;
use crate::routes::bury_card;
use crate::routes::close_collection;
use crate::routes::create_deck;
use crate::routes::create_note;
use crate::routes::delete_card;
use crate::routes::delete_deck;
use crate::routes::delete_note;
use crate::routes::flag_card;
use crate::routes::get_card;
use crate::routes::get_collection_info;
use crate::routes::get_deck;
use crate::routes::get_deck_tree;
use crate::routes::get_note;
use crate::routes::get_note_cards;
use crate::routes::login;
use crate::routes::logout;
use crate::routes::me;
use crate::routes::register;
use crate::routes::suspend_card;
use crate::routes::unsuspend_card;
use crate::routes::update_card;
use crate::routes::update_note;
use crate::routes::AuthRouteState;
use crate::swagger_ui;
use crate::WebAppConfig;

pub fn create_router(config: &WebAppConfig, auth_state: AuthState) -> Router {
    // Routes that require authentication
    let protected_routes = Router::new()
        .route("/api/v1/auth/logout", post(logout))
        .route("/api/v1/auth/me", get(me))
        .route("/api/v1/auth/profile", get(me))
        .route("/api/v1/collection", get(get_collection_info))
        .route("/api/v1/collection/info", get(get_collection_info))
        .route("/api/v1/collection/close", post(close_collection))
        .route("/api/v1/decks", get(get_deck_tree))
        .route("/api/v1/decks", post(create_deck))
        .route("/api/v1/decks/{id}", get(get_deck))
        .route("/api/v1/decks/{id}", delete(delete_deck))
        .route("/api/v1/notes", post(create_note))
        .route("/api/v1/notes/{id}", get(get_note))
        .route("/api/v1/notes/{id}", put(update_note))
        .route("/api/v1/notes/{id}", delete(delete_note))
        .route("/api/v1/notes/{id}/cards", get(get_note_cards))
        .route("/api/v1/cards/{id}", get(get_card))
        .route("/api/v1/cards/{id}", put(update_card))
        .route("/api/v1/cards/{id}", delete(delete_card))
        .route("/api/v1/cards/{id}/flag", post(flag_card))
        .route("/api/v1/cards/{id}/suspend", post(suspend_card))
        .route("/api/v1/cards/{id}/unsuspend", post(unsuspend_card))
        .route("/api/v1/cards/{id}/bury", post(bury_card))
        .route("/api/v1/cards/batch", post(batch_get_cards))
        .route("/api/v1/cards/batch-update", post(batch_update_cards))
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
        .route("/api-docs/openapi.json", get(openapi_spec_handler))
        .route("/swagger-ui", get(swagger_ui::swagger_ui_handler))
        .route("/swagger-ui/", get(swagger_ui::swagger_ui_handler))
        .route("/api/v1/auth/register", post(register))
        .route("/api/v1/auth/login", post(login));

    // Combine all routes with state
    public_routes
        .merge(protected_routes)
        .with_state(auth_route_state)
}

async fn openapi_spec_handler() -> Json<serde_json::Value> {
    Json(openapi::openapi_spec())
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
        .swagger-link {
            display: inline-block;
            background: #0a84ff;
            color: white;
            padding: 10px 20px;
            text-decoration: none;
            border-radius: 5px;
            margin: 20px 0;
        }
        .swagger-link:hover {
            background: #0066cc;
        }
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
    
    <a href="/swagger-ui" class="swagger-link">
        📚 View API Documentation (Swagger UI)
    </a>
    
    <h2>Quick Links</h2>
    <ul>
        <li><a href="/swagger-ui">Swagger UI</a> - Interactive API documentation</li>
        <li><a href="/api-docs/openapi.json">OpenAPI Spec (JSON)</a> - Machine-readable API specification</li>
        <li><a href="/health">Health Check</a> - Server status</li>
        <li><a href="/api/v1/info">API Info</a> - Server information (JSON)</li>
    </ul>
    
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
        <li><code>GET /api/v1/auth/profile</code> - Get current user info (alias)</li>
        <li><code>POST /api/v1/auth/logout</code> - Logout user</li>
        <li><code>GET /api/v1/collection</code> - Get collection info</li>
        <li><code>POST /api/v1/collection/close</code> - Close collection</li>
        <li><code>GET /api/v1/decks</code> - Get deck tree</li>
        <li><code>POST /api/v1/decks</code> - Create deck</li>
        <li><code>GET /api/v1/decks/{id}</code> - Get deck by ID</li>
        <li><code>DELETE /api/v1/decks/{id}</code> - Delete deck</li>
        <li><code>POST /api/v1/notes</code> - Create note</li>
        <li><code>GET /api/v1/notes/{id}</code> - Get note by ID</li>
        <li><code>PUT /api/v1/notes/{id}</code> - Update note</li>
        <li><code>DELETE /api/v1/notes/{id}</code> - Delete note</li>
        <li><code>GET /api/v1/notes/{id}/cards</code> - Get cards for note</li>
        <li><code>GET /api/v1/cards/{id}</code> - Get card by ID</li>
        <li><code>PUT /api/v1/cards/{id}</code> - Update card</li>
        <li><code>DELETE /api/v1/cards/{id}</code> - Delete card</li>
        <li><code>POST /api/v1/cards/{id}/flag</code> - Flag card</li>
        <li><code>POST /api/v1/cards/{id}/suspend</code> - Suspend card</li>
        <li><code>POST /api/v1/cards/{id}/unsuspend</code> - Unsuspend card</li>
        <li><code>POST /api/v1/cards/{id}/bury</code> - Bury card</li>
        <li><code>POST /api/v1/cards/batch</code> - Get multiple cards</li>
        <li><code>POST /api/v1/cards/batch-update</code> - Update multiple cards</li>
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
