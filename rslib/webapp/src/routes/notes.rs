use axum::extract::Path;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Extension;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;

use crate::auth::AuthUser;
use crate::error::Result;
use crate::error::WebAppError;
use crate::routes::AuthRouteState;

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteInfo {
    pub id: i64,
    pub notetype_id: i64,
    pub fields: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateNoteRequest {
    pub deck_id: i64,
    pub notetype_id: i64,
    pub fields: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateNoteResponse {
    pub success: bool,
    pub note_id: i64,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateNoteRequest {
    pub fields: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct CheckNoteFieldsRequest {
    pub notetype_id: i64,
    pub fields: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct CheckNoteFieldsResponse {
    pub state: i32, // 0=NORMAL, 1=EMPTY, 2=DUPLICATE, etc.
}

/// Check note fields for duplicates or errors
pub async fn check_note_fields(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(request): Json<CheckNoteFieldsRequest>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    // Get the notetype
    let notetype = col
        .get_notetype(anki::notetype::NotetypeId(request.notetype_id))
        .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?
        .ok_or_else(|| WebAppError::bad_request("Notetype not found"))?;

    // Create a temporary note
    let mut note = anki::notes::Note::new(&notetype);

    // Set fields
    for (idx, field_value) in request.fields.iter().enumerate() {
        if idx < note.fields().len() {
            note.set_field(idx, field_value.clone())
                .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;
        }
    }

    // Run the check
    let state = col
        .note_fields_check(&note)
        .map_err(|e| WebAppError::internal(&e.to_string()))?;

    drop(col);

    Ok(Json(CheckNoteFieldsResponse { state: state as i32 }))
}

/// Get a note by ID
pub async fn get_note(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(note_id): Path<i64>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let col = backend.lock().unwrap();

    let note = col
        .storage
        .get_note(anki::notes::NoteId(note_id))
        .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?
        .ok_or_else(|| WebAppError::not_found("Note not found"))?;

    let notetype_id = note.notetype_id.0;
    let fields = note.fields().clone();
    let tags = note.tags.clone();

    drop(col);

    Ok(Json(NoteInfo {
        id: note.id.0,
        notetype_id,
        fields,
        tags,
    }))
}

/// Create a new note
pub async fn create_note(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(request): Json<CreateNoteRequest>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    // Get the notetype
    let notetype = col
        .get_notetype(anki::notetype::NotetypeId(request.notetype_id))
        .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?
        .ok_or_else(|| WebAppError::bad_request("Notetype not found"))?;

    // Create a new note
    let mut note = anki::notes::Note::new(&notetype);

    // Set fields
    for (idx, field_value) in request.fields.iter().enumerate() {
        if idx < note.fields().len() {
            note.set_field(idx, field_value.clone())
                .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;
        }
    }

    // Set tags
    note.tags = request.tags;

    // Add the note to the collection
    let output = col
        .add_note(&mut note, anki::decks::DeckId(request.deck_id))
        .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;

    let note_id = note.id.0;

    drop(col);

    Ok(Json(CreateNoteResponse {
        success: true,
        note_id,
        message: format!(
            "Note created successfully ({} cards generated)",
            output.output
        ),
    }))
}

/// Update a note
pub async fn update_note(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(note_id): Path<i64>,
    Json(request): Json<UpdateNoteRequest>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    // Get the existing note
    let mut note = col
        .storage
        .get_note(anki::notes::NoteId(note_id))
        .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?
        .ok_or_else(|| WebAppError::not_found("Note not found"))?;

    // Update fields
    for (idx, field_value) in request.fields.iter().enumerate() {
        if idx < note.fields().len() {
            note.set_field(idx, field_value.clone())
                .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;
        }
    }

    // Update tags
    note.tags = request.tags;

    // Update the note
    col.update_note(&mut note)
        .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;

    drop(col);

    Ok(Json(MessageResponse {
        success: true,
        message: "Note updated successfully".to_string(),
    }))
}

/// Delete a note
pub async fn delete_note(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(note_id): Path<i64>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    // Remove the note
    let output = col
        .remove_notes(&[anki::notes::NoteId(note_id)])
        .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;

    drop(col);

    Ok(Json(MessageResponse {
        success: true,
        message: format!(
            "Note deleted successfully ({} cards removed)",
            output.output
        ),
    }))
}

/// Get cards for a note
pub async fn get_note_cards(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(note_id): Path<i64>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let col = backend.lock().unwrap();

    let cards = col
        .storage
        .all_cards_of_note(anki::notes::NoteId(note_id))
        .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;

    drop(col);

    Ok(Json(serde_json::json!({
        "card_ids": cards.into_iter().map(|card| card.id().0).collect::<Vec<_>>(),
    })))
}
