use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

use crate::auth::jwt::JwtManager;
use crate::db::Database;
use crate::error::WebAppError;

#[derive(Clone)]
pub struct AuthState {
    pub jwt_manager: Arc<JwtManager>,
    pub database: Arc<Database>,
}

/// Extension type to hold authenticated user info
#[derive(Clone, Debug)]
pub struct AuthUser {
    pub user_id: i64,
    pub username: String,
    pub session_id: String,
}

/// Middleware to require authentication for protected routes
pub async fn require_auth(
    State(state): State<AuthState>,
    mut request: Request,
    next: Next,
) -> Result<Response, WebAppError> {
    // Extract token from Authorization header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| WebAppError::unauthorized("Missing authorization header"))?;

    // Parse "Bearer <token>"
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| WebAppError::unauthorized("Invalid authorization header format"))?;

    // Verify JWT
    let claims = state
        .jwt_manager
        .verify_token(token)
        .map_err(|e| WebAppError::unauthorized(&format!("Invalid token: {}", e)))?;

    // Verify session is still valid in database
    let session = state
        .database
        .sessions()
        .get(&claims.session_id)
        .map_err(|e| WebAppError::internal(&format!("Database error: {}", e)))?
        .ok_or_else(|| WebAppError::unauthorized("Session not found or expired"))?;

    if session.is_expired() {
        return Err(WebAppError::unauthorized("Session has expired"));
    }

    // Update session last accessed time
    state
        .database
        .sessions()
        .update_access_time(&claims.session_id)
        .map_err(|e| WebAppError::internal(&format!("Failed to update session: {}", e)))?;

    // Store auth info in request extensions
    let auth_user = AuthUser {
        user_id: claims.user_id().map_err(|e| WebAppError::internal(&e.to_string()))?,
        username: claims.username.clone(),
        session_id: claims.session_id.clone(),
    };

    request.extensions_mut().insert(auth_user);

    Ok(next.run(request).await)
}

/// Optional auth middleware - doesn't fail if no auth is provided
pub async fn optional_auth(
    State(state): State<AuthState>,
    mut request: Request,
    next: Next,
) -> Response {
    // Try to extract and verify token
    if let Some(auth_header) = request.headers().get(header::AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                if let Ok(claims) = state.jwt_manager.verify_token(token) {
                    if let Ok(Some(session)) = state.database.sessions().get(&claims.session_id) {
                        if !session.is_expired() {
                            if let Ok(user_id) = claims.user_id() {
                                let auth_user = AuthUser {
                                    user_id,
                                    username: claims.username.clone(),
                                    session_id: claims.session_id.clone(),
                                };
                                request.extensions_mut().insert(auth_user);
                                
                                // Update session access time (ignore errors)
                                let _ = state.database.sessions().update_access_time(&claims.session_id);
                            }
                        }
                    }
                }
            }
        }
    }

    next.run(request).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::jwt::Claims;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        middleware,
        routing::get,
        Extension, Router,
    };
    use tower::ServiceExt as _;

    async fn protected_handler(Extension(user): Extension<AuthUser>) -> String {
        format!("Hello, {}! User ID: {}", user.username, user.user_id)
    }

    async fn setup_test_state() -> AuthState {
        let db = Arc::new(Database::open(":memory:").unwrap());
        db.initialize().unwrap();
        
        // Create test user
        db.users().create("testuser", "hash", None).unwrap();
        
        let jwt_manager = Arc::new(JwtManager::new("test_secret"));
        
        AuthState {
            jwt_manager,
            database: db,
        }
    }

    #[tokio::test]
    async fn test_require_auth_success() {
        let state = setup_test_state().await;
        
        // Create session for test user
        let session = state.database.sessions().create("sess_123", 1, 3600).unwrap();
        
        // Generate token
        let claims = Claims::new(1, "testuser".to_string(), session.id.clone(), 1);
        let token = state.jwt_manager.generate_token(&claims).unwrap();
        
        // Create router with auth middleware
        let app = Router::new()
            .route("/protected", get(protected_handler))
            .layer(middleware::from_fn_with_state(state.clone(), require_auth))
            .with_state(state);
        
        // Make request with valid token
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/protected")
                    .header("Authorization", format!("Bearer {}", token))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_require_auth_no_header() {
        let state = setup_test_state().await;
        
        let app = Router::new()
            .route("/protected", get(protected_handler))
            .layer(middleware::from_fn_with_state(state.clone(), require_auth))
            .with_state(state);
        
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/protected")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
