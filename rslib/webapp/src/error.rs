// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use std::fmt;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use serde_json::json;

pub type Result<T> = std::result::Result<T, WebAppError>;

#[derive(Debug)]
pub enum WebAppError {
    Internal(String),
    BadRequest(String),
    Unauthorized(String),
    NotFound(String),
    Conflict(String),
    Forbidden(String),
}

impl WebAppError {
    pub fn internal(msg: &str) -> Self {
        WebAppError::Internal(msg.to_string())
    }

    pub fn bad_request(msg: &str) -> Self {
        WebAppError::BadRequest(msg.to_string())
    }

    pub fn unauthorized(msg: &str) -> Self {
        WebAppError::Unauthorized(msg.to_string())
    }

    pub fn not_found(msg: &str) -> Self {
        WebAppError::NotFound(msg.to_string())
    }

    pub fn conflict(msg: &str) -> Self {
        WebAppError::Conflict(msg.to_string())
    }

    pub fn forbidden(msg: &str) -> Self {
        WebAppError::Forbidden(msg.to_string())
    }

    pub fn not_implemented(msg: &str) -> Self {
        WebAppError::Internal(format!("Not implemented: {}", msg))
    }
}

impl fmt::Display for WebAppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WebAppError::Internal(msg) => write!(f, "Internal error: {}", msg),
            WebAppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            WebAppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            WebAppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            WebAppError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            WebAppError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
        }
    }
}

impl std::error::Error for WebAppError {}

impl IntoResponse for WebAppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            WebAppError::Internal(msg) => {
                // Log internal errors with full context
                tracing::error!("Internal server error: {}", msg);
                // Temporarily expose message for debugging the "object Object" issue
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Internal error: {}", msg),
                )
            }
            WebAppError::BadRequest(msg) => {
                tracing::warn!("Bad request: {}", msg);
                (StatusCode::BAD_REQUEST, msg.clone())
            }
            WebAppError::Unauthorized(msg) => {
                tracing::warn!("Unauthorized access attempt: {}", msg);
                (StatusCode::UNAUTHORIZED, msg.clone())
            }
            WebAppError::NotFound(msg) => {
                tracing::debug!("Not found: {}", msg);
                (StatusCode::NOT_FOUND, msg.clone())
            }
            WebAppError::Conflict(msg) => {
                tracing::warn!("Conflict: {}", msg);
                (StatusCode::CONFLICT, msg.clone())
            }
            WebAppError::Forbidden(msg) => {
                tracing::warn!("Forbidden access attempt: {}", msg);
                (StatusCode::FORBIDDEN, msg.clone())
            }
        };

        let body = Json(json!({
            "success": false,
            "error": {
                "message": error_message,
            }
        }));

        (status, body).into_response()
    }
}

impl From<anyhow::Error> for WebAppError {
    fn from(err: anyhow::Error) -> Self {
        WebAppError::Internal(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum::response::IntoResponse;
    use serde_json::Value;

    use super::*;

    async fn response_to_json(response: Response) -> Value {
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        serde_json::from_slice(&body).unwrap()
    }

    #[tokio::test]
    async fn test_internal_error_response() {
        let error = WebAppError::internal("Database connection failed");
        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let json = response_to_json(response).await;
        assert_eq!(json["success"], false);
        // Internal errors should not expose details
        assert_eq!(json["error"]["message"], "Internal server error");
    }

    #[tokio::test]
    async fn test_bad_request_response() {
        let error = WebAppError::bad_request("Invalid username format");
        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let json = response_to_json(response).await;
        assert_eq!(json["success"], false);
        assert_eq!(json["error"]["message"], "Invalid username format");
    }

    #[tokio::test]
    async fn test_unauthorized_response() {
        let error = WebAppError::unauthorized("Invalid credentials");
        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        let json = response_to_json(response).await;
        assert_eq!(json["success"], false);
        assert_eq!(json["error"]["message"], "Invalid credentials");
    }

    #[tokio::test]
    async fn test_not_found_response() {
        let error = WebAppError::not_found("User not found");
        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let json = response_to_json(response).await;
        assert_eq!(json["success"], false);
        assert_eq!(json["error"]["message"], "User not found");
    }

    #[tokio::test]
    async fn test_conflict_response() {
        let error = WebAppError::conflict("Username already exists");
        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::CONFLICT);

        let json = response_to_json(response).await;
        assert_eq!(json["success"], false);
        assert_eq!(json["error"]["message"], "Username already exists");
    }

    #[tokio::test]
    async fn test_forbidden_response() {
        let error = WebAppError::forbidden("Access denied");
        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::FORBIDDEN);

        let json = response_to_json(response).await;
        assert_eq!(json["success"], false);
        assert_eq!(json["error"]["message"], "Access denied");
    }

    #[tokio::test]
    async fn test_anyhow_error_conversion() {
        let anyhow_err = anyhow::anyhow!("Something went wrong");
        let webapp_err: WebAppError = anyhow_err.into();

        let response = webapp_err.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let json = response_to_json(response).await;
        assert_eq!(json["success"], false);
        // Anyhow errors are treated as internal errors
        assert_eq!(json["error"]["message"], "Internal server error");
    }

    #[test]
    fn test_error_display() {
        assert_eq!(
            WebAppError::internal("test").to_string(),
            "Internal error: test"
        );
        assert_eq!(
            WebAppError::bad_request("test").to_string(),
            "Bad request: test"
        );
        assert_eq!(
            WebAppError::unauthorized("test").to_string(),
            "Unauthorized: test"
        );
    }

    #[test]
    fn test_helper_constructors() {
        let err = WebAppError::internal("test");
        assert!(matches!(err, WebAppError::Internal(_)));

        let err = WebAppError::bad_request("test");
        assert!(matches!(err, WebAppError::BadRequest(_)));

        let err = WebAppError::unauthorized("test");
        assert!(matches!(err, WebAppError::Unauthorized(_)));

        let err = WebAppError::not_found("test");
        assert!(matches!(err, WebAppError::NotFound(_)));

        let err = WebAppError::conflict("test");
        assert!(matches!(err, WebAppError::Conflict(_)));

        let err = WebAppError::forbidden("test");
        assert!(matches!(err, WebAppError::Forbidden(_)));
    }
}
