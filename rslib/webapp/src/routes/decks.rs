use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension, Json,
};
use serde::{Deserialize, Serialize};

use crate::auth::AuthUser;
use crate::error::{Result, WebAppError};
use crate::routes::AuthRouteState;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeckTree {
    pub decks: Vec<DeckNode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeckNode {
    pub id: i64,
    pub name: String,
    pub collapsed: bool,
    pub children: Vec<DeckNode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDeckRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeckRequest {
    pub name: Option<String>,
    pub collapsed: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct DeckInfo {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub success: bool,
    pub message: String,
    pub id: Option<i64>,
}

/// Get deck tree for current user
pub async fn get_deck_tree(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();
    
    // Get deck tree from collection
    let tree = col.deck_tree(Default::default())
        .map_err(|e| WebAppError::internal(&e.to_string()))?;
    
    drop(col);

    // Convert to our response format
    let deck_tree = convert_deck_tree(tree);

    Ok(Json(deck_tree))
}

/// Create a new deck
pub async fn create_deck(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(request): Json<CreateDeckRequest>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();
    
    // Create the deck
    let deck = col.get_or_create_normal_deck(&request.name)
        .map_err(|e| WebAppError::internal(&e.to_string()))?;
    let deck_id = deck.id.0;
    
    drop(col);

    Ok(Json(MessageResponse {
        success: true,
        message: format!("Deck '{}' created", request.name),
        id: Some(deck_id),
    }))
}

/// Get deck by ID
pub async fn get_deck(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(deck_id): Path<i64>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();
    
    let deck_id = anki::decks::DeckId(deck_id);
    let deck = col.get_deck(deck_id)
        .map_err(|e| WebAppError::internal(&e.to_string()))?
        .ok_or_else(|| WebAppError::not_found("Deck not found"))?;
    
    drop(col);

    Ok(Json(DeckInfo {
        id: deck.id.0,
        name: deck.name.to_string(),
    }))
}

/// Delete a deck
pub async fn delete_deck(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(deck_id): Path<i64>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();
    
    let deck_ids = vec![anki::decks::DeckId(deck_id)];
    col.remove_decks_and_child_decks(&deck_ids)
        .map_err(|e| WebAppError::internal(&e.to_string()))?;
    
    drop(col);

    Ok(Json(MessageResponse {
        success: true,
        message: "Deck deleted successfully".to_string(),
        id: None,
    }))
}

// Helper function to convert protobuf tree to our format
fn convert_deck_tree(tree: anki_proto::decks::DeckTreeNode) -> DeckTree {
    fn convert_node(node: anki_proto::decks::DeckTreeNode) -> DeckNode {
        DeckNode {
            id: node.deck_id,
            name: node.name,
            collapsed: node.collapsed,
            children: node.children.into_iter().map(convert_node).collect(),
        }
    }

    DeckTree {
        decks: vec![convert_node(tree)],
    }
}
