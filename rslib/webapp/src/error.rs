// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use std::fmt;

pub type Result<T> = std::result::Result<T, WebAppError>;

#[derive(Debug)]
pub enum WebAppError {
    Internal(String),
    BadRequest(String),
    Unauthorized(String),
    NotFound(String),
    Conflict(String),
}

impl fmt::Display for WebAppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WebAppError::Internal(msg) => write!(f, "Internal error: {}", msg),
            WebAppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            WebAppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            WebAppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            WebAppError::Conflict(msg) => write!(f, "Conflict: {}", msg),
        }
    }
}

impl std::error::Error for WebAppError {}

impl IntoResponse for WebAppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            WebAppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            WebAppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            WebAppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            WebAppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            WebAppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
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
