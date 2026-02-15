use axum::body::Bytes;
use axum::extract::Multipart;
use axum::extract::Path;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Extension;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;

use anki::services::MediaService;
use crate::auth::AuthUser;
use crate::error::Result;
use crate::error::WebAppError;
use crate::routes::AuthRouteState;

#[derive(Debug, Serialize)]
pub struct CheckMediaResponse {
    pub unused: Vec<String>,
    pub missing: Vec<String>,
    pub missing_media_notes: Vec<i64>,
    pub report: String,
    pub have_trash: bool,
}

#[derive(Debug, Serialize)]
pub struct AddMediaResponse {
    pub success: bool,
    pub filename: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteMediaRequest {
    pub filenames: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct DeleteMediaResponse {
    pub success: bool,
    pub message: String,
}

/// Check media files (find unused and missing files)
pub async fn check_media(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    // Check media
    let result = col
        .check_media()
        .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;

    drop(col);

    Ok(Json(CheckMediaResponse {
        unused: result.unused,
        missing: result.missing,
        missing_media_notes: result.missing_media_notes,
        report: result.report,
        have_trash: result.have_trash,
    }))
}

/// Get a media file  
/// Note: This endpoint requires access to the media folder path which is currently private.
/// For now, media files should be accessed through the desktop app or by implementing
/// a public accessor method in the Collection struct.
pub async fn get_media(
    State(_state): State<AuthRouteState>,
    Extension(_auth_user): Extension<AuthUser>,
    Path(_filename): Path<String>,
) -> Result<Response> {
    // TODO: Implement once Collection provides public access to media_folder
    Err(WebAppError::not_implemented(
        "Media file download not yet implemented - use add_media and check_media endpoints",
    ))
}

/// Upload a media file
pub async fn add_media(
    State(state): State<AuthRouteState>,
    auth_user: Extension<AuthUser>,
    multipart: Multipart,
) -> Result<impl IntoResponse> {
    let Extension(user) = auth_user;
    
    // Process multipart first to extract file data
    let (desired_name, file_data) = extract_file_from_multipart(multipart).await?;
    
    let backend = state
        .backend_manager
        .get_or_create_backend(user.user_id, &user.username)?;

    let mut col = backend.lock().unwrap();

    // Add file using the service
    let chosen_name = col
        .add_media_file(anki_proto::media::AddMediaFileRequest {
            desired_name: desired_name.clone(),
            data: file_data.to_vec(),
        })
        .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;

    drop(col);

    Ok(Json(AddMediaResponse {
        success: true,
        filename: chosen_name.val,
    }))
}

async fn extract_file_from_multipart(mut multipart: Multipart) -> Result<(String, Bytes)> {
    let mut filename: Option<String> = None;
    let mut file_data: Option<Bytes> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| WebAppError::bad_request(&format!("Invalid multipart data: {}", e)))?
    {
        let field_name = field.name().unwrap_or("").to_string();

        if field_name == "file" {
            filename = field.file_name().map(|s| s.to_string());
            file_data = Some(
                field
                    .bytes()
                    .await
                    .map_err(|e| WebAppError::bad_request(&format!("Failed to read file: {}", e)))?,
            );
        }
    }

    let filename = filename.ok_or_else(|| WebAppError::bad_request("No file provided"))?;
    let file_data = file_data.ok_or_else(|| WebAppError::bad_request("No file data"))?;

    Ok((filename, file_data))
}

/// Delete media files (move to trash)
pub async fn delete_media(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(request): Json<DeleteMediaRequest>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    // Trash files
    col
        .trash_media_files(anki_proto::media::TrashMediaFilesRequest {
            fnames: request.filenames.clone(),
        })
        .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;

    let count = request.filenames.len();

    drop(col);

    Ok(Json(DeleteMediaResponse {
        success: true,
        message: format!("Moved {} file(s) to trash", count),
    }))
}
