use axum::{extract::State, response::IntoResponse, Extension, Json};
use serde::{Deserialize, Serialize};

use crate::auth::AuthUser;
use crate::error::Result;
use crate::routes::AuthRouteState;

#[derive(Debug, Serialize)]
pub struct CollectionInfo {
    pub user_id: i64,
    pub username: String,
    pub backend_active: bool,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub success: bool,
    pub message: String,
}

/// Get collection info for the current user
pub async fn get_collection_info(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<impl IntoResponse> {
    // Get or create backend for this user
    let _backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    Ok(Json(CollectionInfo {
        user_id: auth_user.user_id,
        username: auth_user.username.clone(),
        backend_active: true,
        message: format!("Collection active for user {}", auth_user.username),
    }))
}

/// Close the collection for the current user
pub async fn close_collection(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<impl IntoResponse> {
    // Close backend for this user
    state.backend_manager.close_backend(auth_user.user_id)?;

    Ok(Json(MessageResponse {
        success: true,
        message: "Collection closed successfully".to_string(),
    }))
}
