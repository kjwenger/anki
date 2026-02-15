use axum::extract::Path;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Extension;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;

use anki::services::TagsService;
use crate::auth::AuthUser;
use crate::error::Result;
use crate::error::WebAppError;
use crate::routes::AuthRouteState;

#[derive(Debug, Serialize)]
pub struct TagsListResponse {
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct TagTreeNode {
    pub name: String,
    pub children: Vec<TagTreeNode>,
    pub level: u32,
    pub collapsed: bool,
}

#[derive(Debug, Serialize)]
pub struct TagTreeResponse {
    pub root: Option<TagTreeNode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RenameTagRequest {
    pub old_name: String,
    pub new_name: String,
}

#[derive(Debug, Serialize)]
pub struct RenameTagResponse {
    pub success: bool,
    pub message: String,
    pub count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteTagRequest {
    pub tag: String,
}

#[derive(Debug, Serialize)]
pub struct DeleteTagResponse {
    pub success: bool,
    pub message: String,
    pub count: usize,
}

#[derive(Debug, Serialize)]
pub struct ClearUnusedTagsResponse {
    pub success: bool,
    pub message: String,
    pub removed_count: usize,
}

/// Get all tags
pub async fn get_tags(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    // Get all tags
    let result = col
        .all_tags()
        .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;

    drop(col);

    Ok(Json(TagsListResponse { tags: result.vals }))
}

/// Get tag tree structure
pub async fn get_tag_tree(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    // Get tag tree
    let result = col
        .tag_tree()
        .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;

    drop(col);

    // Convert protobuf TagTreeNode to our response type
    let root = Some(convert_tag_tree_node(result));

    Ok(Json(TagTreeResponse { root }))
}

fn convert_tag_tree_node(node: anki_proto::tags::TagTreeNode) -> TagTreeNode {
    TagTreeNode {
        name: node.name,
        children: node.children.into_iter().map(convert_tag_tree_node).collect(),
        level: node.level,
        collapsed: node.collapsed,
    }
}

/// Rename a tag
pub async fn rename_tag(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(request): Json<RenameTagRequest>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    // Rename tag
    let result = col
        .rename_tags(anki_proto::tags::RenameTagsRequest {
            current_prefix: request.old_name.clone(),
            new_prefix: request.new_name.clone(),
        })
        .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;

    let count = result.count as usize;

    drop(col);

    Ok(Json(RenameTagResponse {
        success: true,
        message: format!("Renamed tag in {} note(s)", count),
        count,
    }))
}

/// Delete/remove a tag
pub async fn delete_tag(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(tag_name): Path<String>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    // Remove tag
    let result = col
        .remove_tags(&tag_name)
        .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;

    let count = result.output;

    drop(col);

    Ok(Json(DeleteTagResponse {
        success: true,
        message: format!("Removed tag from {} note(s)", count),
        count,
    }))
}

/// Clear unused tags
pub async fn clear_unused_tags(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    // Clear unused tags
    let result = col
        .clear_unused_tags()
        .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;

    let removed_count = result.output;

    drop(col);

    Ok(Json(ClearUnusedTagsResponse {
        success: true,
        message: format!("Removed {} unused tag(s)", removed_count),
        removed_count,
    }))
}
