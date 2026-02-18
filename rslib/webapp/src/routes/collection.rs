use axum::extract::State;
use axum::response::IntoResponse;
use axum::Extension;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;

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

#[derive(Debug, Serialize)]
pub struct Collection {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Serialize)]
pub struct CollectionsResponse {
    pub collections: Vec<Collection>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCollectionRequest {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateCollectionResponse {
    pub path: String,
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

/// List all collections for the current user
/// Note: Currently each user has one collection, so this returns a single-item
/// array
pub async fn list_collections(
    Extension(auth_user): Extension<AuthUser>,
) -> Result<impl IntoResponse> {
    // In the current implementation, each user has one collection
    let collection = Collection {
        name: format!("{}'s Collection", auth_user.username),
        path: format!("user_{}", auth_user.user_id),
    };

    Ok(Json(CollectionsResponse {
        collections: vec![collection],
    }))
}

/// Create a new collection
/// Note: Currently this is a no-op since users have one collection
pub async fn create_collection(
    Extension(auth_user): Extension<AuthUser>,
    Json(payload): Json<CreateCollectionRequest>,
) -> Result<impl IntoResponse> {
    // In the current implementation, we just acknowledge the request
    // The user's collection already exists
    let path = format!(
        "user_{}_{}",
        auth_user.user_id,
        payload.name.replace(' ', "_")
    );

    Ok(Json(CreateCollectionResponse {
        path,
        message: format!("Collection '{}' is ready", payload.name),
    }))
}

/// Delete a collection
/// Note: Currently this is a no-op to prevent users from deleting their main
/// collection
pub async fn delete_collection() -> Result<impl IntoResponse> {
    Ok(Json(MessageResponse {
        success: false,
        message: "Cannot delete the main collection".to_string(),
    }))
}
