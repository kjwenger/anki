use axum::extract::State;
use axum::response::IntoResponse;
use axum::Extension;
use axum::Json;
use serde::Serialize;

use crate::auth::AuthUser;
use crate::error::Result;
use crate::error::WebAppError;
use crate::routes::AuthRouteState;

#[derive(Debug, Serialize)]
pub struct NotetypeInfo {
    pub id: i64,
    pub name: String,
    pub fields: Vec<NotetypeField>,
    pub templates: Vec<NotetypeTemplate>,
}

#[derive(Debug, Serialize)]
pub struct NotetypeField {
    pub name: String,
    pub ord: u32,
}

#[derive(Debug, Serialize)]
pub struct NotetypeTemplate {
    pub name: String,
    pub ord: u32,
}

#[derive(Debug, Serialize)]
pub struct NotetypeListItem {
    pub id: i64,
    pub name: String,
}

/// List all notetypes
pub async fn list_notetypes(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    let notetypes = col
        .get_all_notetypes()
        .map_err(|e| WebAppError::internal(&e.to_string()))?;

    let mut notetype_list: Vec<NotetypeListItem> = notetypes
        .into_iter()
        .map(|notetype| NotetypeListItem {
            id: notetype.id.0,
            name: notetype.name.clone(),
        })
        .collect();

    // Sort by name for consistent ordering
    notetype_list.sort_by(|a, b| a.name.cmp(&b.name));

    drop(col);

    Ok(Json(serde_json::json!({
        "notetypes": notetype_list
    })))
}

/// Get a specific notetype by ID
pub async fn get_notetype(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    axum::extract::Path(notetype_id): axum::extract::Path<i64>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    let notetype = col
        .get_notetype(anki::notetype::NotetypeId(notetype_id))
        .map_err(|e| WebAppError::internal(&e.to_string()))?
        .ok_or_else(|| WebAppError::not_found("Notetype not found"))?;

    let fields: Vec<NotetypeField> = notetype
        .fields
        .iter()
        .map(|f| NotetypeField {
            name: f.name.clone(),
            ord: f.ord.unwrap_or(0),
        })
        .collect();

    let templates: Vec<NotetypeTemplate> = notetype
        .templates
        .iter()
        .map(|t| NotetypeTemplate {
            name: t.name.clone(),
            ord: t.ord.unwrap_or(0),
        })
        .collect();

    drop(col);

    Ok(Json(NotetypeInfo {
        id: notetype.id.0,
        name: notetype.name.clone(),
        fields,
        templates,
    }))
}
