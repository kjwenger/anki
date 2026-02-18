// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use axum::http::HeaderValue;
use axum::http::Method;
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
use tower_http::cors::CorsLayer;

use crate::auth::require_auth;
use crate::auth::AuthState;
use crate::openapi;
use crate::routes::add_media;
use crate::routes::answer_card;
use crate::routes::batch_get_cards;
use crate::routes::batch_update_cards;
use crate::routes::bury_card;
use crate::routes::check_media;
use crate::routes::check_note_fields;
use crate::routes::clear_unused_tags;
use crate::routes::close_collection;
use crate::routes::create_collection;
use crate::routes::create_deck;
use crate::routes::create_note;
use crate::routes::delete_card;
use crate::routes::delete_collection;
use crate::routes::delete_deck;
use crate::routes::delete_media;
use crate::routes::delete_note;
use crate::routes::delete_tag;
use crate::routes::find_and_replace;
use crate::routes::flag_card;
use crate::routes::get_card;
use crate::routes::get_card_stats;
use crate::routes::get_collection_info;
use crate::routes::get_collection_stats;
use crate::routes::get_deck;
use crate::routes::get_deck_counts;
use crate::routes::get_deck_tree;
use crate::routes::get_graphs;
use crate::routes::get_media;
use crate::routes::get_next_card;
use crate::routes::get_next_states;
use crate::routes::get_note;
use crate::routes::get_note_cards;
use crate::routes::get_notetype;
use crate::routes::get_tag_tree;
use crate::routes::get_tags;
use crate::routes::get_today_stats;
use crate::routes::list_collections;
use crate::routes::list_notetypes;
use crate::routes::login;
use crate::routes::logout;
use crate::routes::me;
use crate::routes::redo;
use crate::routes::register;
use crate::routes::rename_tag;
use crate::routes::search_cards;
use crate::routes::search_notes;
use crate::routes::suspend_card;
use crate::routes::undo;
use crate::routes::unsuspend_card;
use crate::routes::update_card;
use crate::routes::update_deck;
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
        .route("/api/v1/collections", get(list_collections))
        .route("/api/v1/collections", post(create_collection))
        .route("/api/v1/collections/{path}", delete(delete_collection))
        .route("/api/v1/decks", get(get_deck_tree))
        .route("/api/v1/decks", post(create_deck))
        .route("/api/v1/decks/{id}", get(get_deck))
        .route("/api/v1/decks/{id}", put(update_deck))
        .route("/api/v1/decks/{id}", delete(delete_deck))
        .route("/api/v1/notes", post(create_note))
        .route("/api/v1/notes/check-fields", post(check_note_fields))
        .route("/api/v1/notes/{id}", get(get_note))
        .route("/api/v1/notes/{id}", put(update_note))
        .route("/api/v1/notes/{id}", delete(delete_note))
        .route("/api/v1/notes/{id}/cards", get(get_note_cards))
        .route("/api/v1/notetypes", get(list_notetypes))
        .route("/api/v1/notetypes/{id}", get(get_notetype))
        .route("/api/v1/cards/{id}", get(get_card))
        .route("/api/v1/cards/{id}", put(update_card))
        .route("/api/v1/cards/{id}", delete(delete_card))
        .route("/api/v1/cards/{id}/flag", post(flag_card))
        .route("/api/v1/cards/{id}/suspend", post(suspend_card))
        .route("/api/v1/cards/{id}/unsuspend", post(unsuspend_card))
        .route("/api/v1/cards/{id}/bury", post(bury_card))
        .route("/api/v1/cards/batch", post(batch_get_cards))
        .route("/api/v1/cards/batch-update", post(batch_update_cards))
        .route("/api/v1/search/cards", post(search_cards))
        .route("/api/v1/search/notes", post(search_notes))
        .route("/api/v1/search/find-replace", post(find_and_replace))
        .route("/api/v1/media/check", get(check_media))
        .route("/api/v1/media/{filename}", get(get_media))
        .route("/api/v1/media", post(add_media))
        .route("/api/v1/media", delete(delete_media))
        .route("/api/v1/tags", get(get_tags))
        .route("/api/v1/tags/tree", get(get_tag_tree))
        .route("/api/v1/tags/rename", put(rename_tag))
        .route("/api/v1/tags/{name}", delete(delete_tag))
        .route("/api/v1/tags/clear-unused", post(clear_unused_tags))
        .route("/api/v1/stats/card/{id}", get(get_card_stats))
        .route("/api/v1/stats/collection", get(get_collection_stats))
        .route("/api/v1/stats/graphs", get(get_graphs))
        .route("/api/v1/stats/today", get(get_today_stats))
        .route("/api/v1/scheduler/decks/{deck_id}/next", get(get_next_card))
        .route(
            "/api/v1/scheduler/decks/{deck_id}/cards/{card_id}/answer",
            post(answer_card),
        )
        .route(
            "/api/v1/scheduler/decks/{deck_id}/cards/{card_id}/next-states",
            get(get_next_states),
        )
        .route(
            "/api/v1/scheduler/decks/{deck_id}/counts",
            get(get_deck_counts),
        )
        .route("/api/v1/scheduler/undo", post(undo))
        .route("/api/v1/scheduler/redo", post(redo))
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

    // CORS layer for development (SvelteKit dev server on different port)
    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:5173".parse::<HeaderValue>().unwrap(),
            "http://127.0.0.1:5173".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
        ]);

    // Combine all routes with state
    public_routes
        .merge(protected_routes)
        .with_state(auth_route_state)
        .layer(cors)
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
        <li><code>POST /api/v1/search/cards</code> - Search for cards</li>
        <li><code>POST /api/v1/search/notes</code> - Search for notes</li>
        <li><code>POST /api/v1/search/find-replace</code> - Find and replace in notes</li>
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
