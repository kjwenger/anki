use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Extension;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::auth::hash_password;
use crate::auth::verify_password;
use crate::auth::AuthUser;
use crate::auth::Claims;
use crate::auth::JwtManager;
use crate::db::Database;
use crate::error::Result;
use crate::error::WebAppError;
use crate::session::BackendManager;

#[derive(Clone)]
pub struct AuthRouteState {
    pub database: Arc<Database>,
    pub jwt_manager: Arc<JwtManager>,
    pub backend_manager: Arc<BackendManager>,
    pub session_timeout_hours: i64,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub success: bool,
    pub data: Option<AuthData>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuthData {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: i64,
    pub username: String,
    pub email: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub success: bool,
    pub message: String,
}

/// Register a new user
pub async fn register(
    State(state): State<AuthRouteState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse> {
    // Validate input
    if payload.username.trim().is_empty() {
        return Err(WebAppError::bad_request("Username cannot be empty"));
    }
    if payload.password.len() < 8 {
        return Err(WebAppError::bad_request(
            "Password must be at least 8 characters",
        ));
    }
    if payload.username.len() > 50 {
        return Err(WebAppError::bad_request(
            "Username must be 50 characters or less",
        ));
    }

    // Check if username already exists
    if state
        .database
        .users()
        .get_by_username(&payload.username)?
        .is_some()
    {
        return Err(WebAppError::conflict("Username already exists"));
    }

    // Hash password
    let password_hash = hash_password(&payload.password)
        .map_err(|e| WebAppError::internal(&format!("Failed to hash password: {}", e)))?;

    // Create user
    let user = state.database.users().create(
        &payload.username,
        &password_hash,
        payload.email.as_deref(),
    )?;

    // Create session
    let session_id = Uuid::new_v4().to_string();
    let _session = state.database.sessions().create(
        &session_id,
        user.id,
        state.session_timeout_hours * 3600,
    )?;

    // Generate JWT
    let claims = Claims::new(
        user.id,
        user.username.clone(),
        session_id,
        state.session_timeout_hours,
    );
    let token = state
        .jwt_manager
        .generate_token(&claims)
        .map_err(|e| WebAppError::internal(&format!("Failed to generate token: {}", e)))?;

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            success: true,
            data: Some(AuthData {
                token,
                user: UserInfo {
                    id: user.id,
                    username: user.username,
                    email: user.email,
                },
            }),
            error: None,
        }),
    ))
}

/// Login user
pub async fn login(
    State(state): State<AuthRouteState>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse> {
    // Get user by username
    let user = state
        .database
        .users()
        .get_by_username(&payload.username)?
        .ok_or_else(|| WebAppError::unauthorized("Invalid username or password"))?;

    // Check if user is active
    if !user.is_active {
        return Err(WebAppError::forbidden("Account is disabled"));
    }

    // Verify password
    let is_valid = verify_password(&payload.password, &user.password_hash)
        .map_err(|e| WebAppError::internal(&format!("Password verification error: {}", e)))?;

    if !is_valid {
        return Err(WebAppError::unauthorized("Invalid username or password"));
    }

    // Create session
    let session_id = Uuid::new_v4().to_string();
    let _session = state.database.sessions().create(
        &session_id,
        user.id,
        state.session_timeout_hours * 3600,
    )?;

    // Generate JWT
    let claims = Claims::new(
        user.id,
        user.username.clone(),
        session_id,
        state.session_timeout_hours,
    );
    let token = state
        .jwt_manager
        .generate_token(&claims)
        .map_err(|e| WebAppError::internal(&format!("Failed to generate token: {}", e)))?;

    Ok(Json(AuthResponse {
        success: true,
        data: Some(AuthData {
            token,
            user: UserInfo {
                id: user.id,
                username: user.username,
                email: user.email,
            },
        }),
        error: None,
    }))
}

/// Logout user (invalidate session)
pub async fn logout(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<impl IntoResponse> {
    // Close backend instance for this user
    let _ = state.backend_manager.close_backend(auth_user.user_id);

    // Delete the session
    state.database.sessions().delete(&auth_user.session_id)?;

    Ok(Json(MessageResponse {
        success: true,
        message: "Logged out successfully".to_string(),
    }))
}

/// Get current user info (requires authentication)
pub async fn me(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<impl IntoResponse> {
    let user = state
        .database
        .users()
        .get_by_id(auth_user.user_id)?
        .ok_or_else(|| WebAppError::not_found("User not found"))?;

    Ok(Json(UserInfo {
        id: user.id,
        username: user.username,
        email: user.email,
    }))
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::Request;
    use axum::http::StatusCode;
    use axum::middleware;
    use axum::routing::get;
    use axum::routing::post;
    use axum::Router;
    use tower::ServiceExt;

    use super::*;
    use crate::auth::AuthState;

    async fn setup_test_app() -> (Router, Arc<Database>) {
        let db = Arc::new(Database::open(":memory:").unwrap());
        db.initialize().unwrap();

        let jwt_manager = Arc::new(JwtManager::new("test_secret"));
        let backend_manager = Arc::new(BackendManager::new(std::env::temp_dir()));

        let auth_state = AuthRouteState {
            database: db.clone(),
            jwt_manager: jwt_manager.clone(),
            backend_manager: backend_manager.clone(),
            session_timeout_hours: 24,
        };

        let middleware_state = AuthState {
            database: db.clone(),
            jwt_manager,
            backend_manager,
        };

        let app = Router::new()
            .route("/api/v1/auth/register", post(register))
            .route("/api/v1/auth/login", post(login))
            .route(
                "/api/v1/auth/logout",
                post(logout).layer(middleware::from_fn_with_state(
                    middleware_state.clone(),
                    crate::auth::require_auth,
                )),
            )
            .route(
                "/api/v1/auth/me",
                get(me).layer(middleware::from_fn_with_state(
                    middleware_state,
                    crate::auth::require_auth,
                )),
            )
            .with_state(auth_state);

        (app, db)
    }

    #[tokio::test]
    async fn test_register_success() {
        let (app, _db) = setup_test_app().await;

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/auth/register")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"username":"newuser","password":"password123","email":"test@example.com"}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_register_duplicate_username() {
        let (app, db) = setup_test_app().await;

        // Create user first
        db.users().create("existinguser", "hash", None).unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/auth/register")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"username":"existinguser","password":"password123"}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CONFLICT);
    }

    #[tokio::test]
    async fn test_login_success() {
        let (app, db) = setup_test_app().await;

        // Create user with hashed password
        let hash = hash_password("mypassword").unwrap();
        db.users().create("loginuser", &hash, None).unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/auth/login")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"username":"loginuser","password":"mypassword"}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_login_wrong_password() {
        let (app, db) = setup_test_app().await;

        let hash = hash_password("correctpassword").unwrap();
        db.users().create("testuser", &hash, None).unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/auth/login")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"username":"testuser","password":"wrongpassword"}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
