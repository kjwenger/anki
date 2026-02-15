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
pub struct SearchCardsRequest {
    pub query: String,
    #[serde(default)]
    pub sort_column: Option<String>,
    #[serde(default)]
    pub reverse: bool,
}

#[derive(Debug, Serialize)]
pub struct SearchCardsResponse {
    pub card_ids: Vec<i64>,
    pub count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchNotesRequest {
    pub query: String,
    #[serde(default)]
    pub sort_column: Option<String>,
    #[serde(default)]
    pub reverse: bool,
}

#[derive(Debug, Serialize)]
pub struct SearchNotesResponse {
    pub note_ids: Vec<i64>,
    pub count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FindAndReplaceRequest {
    pub note_ids: Vec<i64>,
    pub search: String,
    pub replacement: String,
    #[serde(default)]
    pub regex: bool,
    #[serde(default)]
    pub match_case: bool,
    #[serde(default)]
    pub field_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FindAndReplaceResponse {
    pub success: bool,
    pub message: String,
    pub replaced_count: usize,
}

/// Search for cards matching a query
pub async fn search_cards(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(request): Json<SearchCardsRequest>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    // Build sort mode
    let sort_mode = if let Some(column) = request.sort_column {
        // Parse column string to Column enum
        use std::str::FromStr;
        match anki::browser_table::Column::from_str(&column) {
            Ok(col) => anki::search::SortMode::Builtin {
                column: col,
                reverse: request.reverse,
            },
            Err(_) => anki::search::SortMode::Custom(column),
        }
    } else {
        anki::search::SortMode::NoOrder
    };

    // Search for cards
    let card_ids = col
        .search_cards(&request.query, sort_mode)
        .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;

    let count = card_ids.len();
    let ids: Vec<i64> = card_ids.into_iter().map(|cid| cid.0).collect();

    drop(col);

    Ok(Json(SearchCardsResponse {
        card_ids: ids,
        count,
    }))
}

/// Search for notes matching a query
pub async fn search_notes(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(request): Json<SearchNotesRequest>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    // Build sort mode
    let sort_mode = if let Some(column) = request.sort_column {
        // Parse column string to Column enum
        use std::str::FromStr;
        match anki::browser_table::Column::from_str(&column) {
            Ok(col) => anki::search::SortMode::Builtin {
                column: col,
                reverse: request.reverse,
            },
            Err(_) => anki::search::SortMode::Custom(column),
        }
    } else {
        anki::search::SortMode::NoOrder
    };

    // Search for notes
    let note_ids = col
        .search_notes(&request.query, sort_mode)
        .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;

    let count = note_ids.len();
    let ids: Vec<i64> = note_ids.into_iter().map(|nid| nid.0).collect();

    drop(col);

    Ok(Json(SearchNotesResponse {
        note_ids: ids,
        count,
    }))
}

/// Find and replace text in note fields
pub async fn find_and_replace(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(request): Json<FindAndReplaceRequest>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    // Convert note IDs
    let nids: Vec<anki::notes::NoteId> = request.note_ids.into_iter().map(anki::notes::NoteId).collect();

    // Build search pattern - if not regex, escape it
    let mut search_pattern = if request.regex {
        request.search.clone()
    } else {
        regex::escape(&request.search)
    };

    // Add case-insensitive flag if needed
    if !request.match_case {
        search_pattern = format!("(?i){}", search_pattern);
    }

    // Perform find and replace
    let result = col
        .find_and_replace(
            nids,
            &search_pattern,
            &request.replacement,
            request.field_name,
        )
        .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;

    let replaced_count = result.output;

    drop(col);

    Ok(Json(FindAndReplaceResponse {
        success: true,
        message: format!("Replaced in {} note(s)", replaced_count),
        replaced_count,
    }))
}
