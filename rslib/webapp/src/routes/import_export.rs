// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use std::io::Write;

use anki::services::ImportExportService;
use axum::extract::Multipart;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Extension;
use axum::Json;
use serde::Serialize;
use tempfile::NamedTempFile;

use crate::auth::AuthUser;
use crate::error::Result;
use crate::error::WebAppError;
use crate::routes::AuthRouteState;

#[derive(Debug, Serialize)]
pub struct WebImportResponse {
    pub success: bool,
    pub message: String,
    pub notes_new: u32,
    pub notes_updated: u32,
}

/// Import an Anki package (.apkg)
pub async fn import_apkg(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse> {
    let mut temp_file = NamedTempFile::new()
        .map_err(|e| WebAppError::internal(&format!("Failed to create temp file: {}", e)))?;

    let mut filename = String::new();
    let mut found_file = false;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| WebAppError::bad_request(&format!("Invalid multipart data: {}", e)))?
    {
        let name = field.name().unwrap_or("").to_string();
        if name == "file" {
            filename = field.file_name().unwrap_or("import.apkg").to_string();
            let data = field
                .bytes()
                .await
                .map_err(|e| WebAppError::bad_request(&format!("Failed to read file: {}", e)))?;
            temp_file
                .write_all(&data)
                .map_err(|e| WebAppError::internal(&format!("Failed to write to temp file: {}", e)))?;
            found_file = true;
        }
    }

    if !found_file {
        return Err(WebAppError::bad_request("No file provided in multipart data"));
    }

    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    let path = temp_file.path().to_string_lossy().to_string();

    let request = anki_proto::import_export::ImportAnkiPackageRequest {
        package_path: path,
        options: Some(anki_proto::import_export::ImportAnkiPackageOptions {
            merge_notetypes: true,
            update_notes: anki_proto::import_export::ImportAnkiPackageUpdateCondition::Always as i32,
            update_notetypes: anki_proto::import_export::ImportAnkiPackageUpdateCondition::Always as i32,
            with_scheduling: true,
            with_deck_configs: true,
        }),
    };

    let response = col
        .import_anki_package(request)
        .map_err(|e| WebAppError::internal(&e.to_string()))?;

    let log = response.log.unwrap_or_default();
    let new_count = log.new.len() as u32;
    let updated_count = log.updated.len() as u32;

    drop(col);

    Ok((
        StatusCode::CREATED,
        Json(WebImportResponse {
            success: true,
            message: format!("Successfully imported '{}'", filename),
            notes_new: new_count,
            notes_updated: updated_count,
        }),
    ))
}
