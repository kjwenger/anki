use anki::services::CardsService;
use anki::services::SchedulerService;
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
pub struct CardInfo {
    pub id: i64,
    pub note_id: i64,
    pub deck_id: i64,
    pub ordinal: u16,
    pub card_type: u8,
    pub queue: i8,
    pub due: i32,
    pub interval: u32,
    pub ease_factor: u16,
    pub reps: u32,
    pub lapses: u32,
    pub flags: u8,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCardRequest {
    pub deck_id: Option<i64>,
    pub due: Option<i32>,
    pub flags: Option<u8>,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct FlagCardRequest {
    pub flag: u8,
}

#[derive(Debug, Deserialize)]
pub struct BatchGetCardsRequest {
    pub card_ids: Vec<i64>,
}

#[derive(Debug, Deserialize)]
pub struct BatchUpdateCardsRequest {
    pub updates: Vec<BatchCardUpdate>,
}

#[derive(Debug, Deserialize)]
pub struct BatchCardUpdate {
    pub card_id: i64,
    pub deck_id: Option<i64>,
    pub due: Option<i32>,
    pub flags: Option<u8>,
}

/// Convert Anki Card (protobuf) to CardInfo
fn card_to_info(card: &anki_proto::cards::Card) -> CardInfo {
    CardInfo {
        id: card.id,
        note_id: card.note_id,
        deck_id: card.deck_id,
        ordinal: card.template_idx as u16,
        card_type: card.ctype as u8,
        queue: card.queue as i8,
        due: card.due,
        interval: card.interval,
        ease_factor: card.ease_factor as u16,
        reps: card.reps,
        lapses: card.lapses,
        flags: card.flags as u8,
    }
}

/// Get a card by ID
pub async fn get_card(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(card_id): Path<i64>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    let card = col
        .get_card(anki_proto::cards::CardId { cid: card_id })
        .map_err(|e: anki::error::AnkiError| {
            if e.to_string().contains("NotFound") {
                WebAppError::not_found("Card not found")
            } else {
                WebAppError::internal(&e.to_string())
            }
        })?;

    let info = card_to_info(&card);

    drop(col);

    Ok(Json(info))
}

/// Update a card
pub async fn update_card(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(card_id): Path<i64>,
    Json(request): Json<UpdateCardRequest>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    // Get the existing card
    let mut card = col
        .get_card(anki_proto::cards::CardId { cid: card_id })
        .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;

    // Update fields if provided
    if let Some(deck_id) = request.deck_id {
        card.deck_id = deck_id;
    }
    if let Some(due) = request.due {
        card.due = due;
    }
    if let Some(flags) = request.flags {
        card.flags = flags as u32;
    }

    // Update the card
    let _ = col
        .update_cards(anki_proto::cards::UpdateCardsRequest {
            cards: vec![card],
            skip_undo_entry: false,
        })
        .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;

    drop(col);

    Ok(Json(MessageResponse {
        success: true,
        message: "Card updated successfully".to_string(),
    }))
}

/// Delete a card
pub async fn delete_card(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(card_id): Path<i64>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    // Check if card exists
    if col.get_card(anki_proto::cards::CardId { cid: card_id }).is_err() {
        drop(col);
        return Err(WebAppError::not_found("Card not found"));
    }

    // Remove the card
    let result = col
        .remove_cards(anki_proto::cards::RemoveCardsRequest {
            card_ids: vec![card_id],
        });
    
    match result {
        Ok(_) => {
            drop(col);
            Ok(Json(MessageResponse {
                success: true,
                message: "Card deleted successfully".to_string(),
            }))
        }
        Err(e) => {
            let err_msg = e.to_string();
            tracing::error!("Failed to remove card {}: {}", card_id, err_msg);
            drop(col);
            Err(WebAppError::internal(&err_msg))
        }
    }
}

/// Flag a card
pub async fn flag_card(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(card_id): Path<i64>,
    Json(request): Json<FlagCardRequest>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    // Set the flag
    let _ = col
        .set_flag(anki_proto::cards::SetFlagRequest {
            card_ids: vec![card_id],
            flag: request.flag as u32,
        })
        .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;

    drop(col);

    Ok(Json(MessageResponse {
        success: true,
        message: "Card flagged successfully".to_string(),
    }))
}

/// Suspend a card
pub async fn suspend_card(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(card_id): Path<i64>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    // Suspend the card
    let _ = <anki::collection::Collection as SchedulerService>::bury_or_suspend_cards(
        &mut col,
        anki_proto::scheduler::BuryOrSuspendCardsRequest {
            card_ids: vec![card_id],
            note_ids: vec![],
            mode: anki_proto::scheduler::bury_or_suspend_cards_request::Mode::Suspend as i32,
        },
    )
    .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;

    drop(col);

    Ok(Json(MessageResponse {
        success: true,
        message: "Card suspended successfully".to_string(),
    }))
}

/// Unsuspend a card
pub async fn unsuspend_card(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(card_id): Path<i64>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    // Unsuspend the card (restore from buried/suspended)
    let _ = col
        .restore_buried_and_suspended_cards(anki_proto::cards::CardIds {
            cids: vec![card_id],
        })
        .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;

    drop(col);

    Ok(Json(MessageResponse {
        success: true,
        message: "Card unsuspended successfully".to_string(),
    }))
}

/// Bury a card
pub async fn bury_card(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(card_id): Path<i64>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    // Bury the card (bury until next day)
    let _ = <anki::collection::Collection as SchedulerService>::bury_or_suspend_cards(
        &mut col,
        anki_proto::scheduler::BuryOrSuspendCardsRequest {
            card_ids: vec![card_id],
            note_ids: vec![],
            mode: anki_proto::scheduler::bury_or_suspend_cards_request::Mode::BuryUser as i32,
        },
    )
    .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;

    drop(col);

    Ok(Json(MessageResponse {
        success: true,
        message: "Card buried successfully".to_string(),
    }))
}

/// Get multiple cards by their IDs (batch get)
pub async fn batch_get_cards(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(request): Json<BatchGetCardsRequest>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    let mut cards = Vec::new();
    for card_id in request.card_ids {
        if let Ok(card) = col.get_card(anki_proto::cards::CardId { cid: card_id }) {
            cards.push(card_to_info(&card));
        }
    }

    drop(col);

    Ok(Json(serde_json::json!({
        "cards": cards,
    })))
}

/// Update multiple cards (batch update)
pub async fn batch_update_cards(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(request): Json<BatchUpdateCardsRequest>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    let mut cards_to_update = Vec::new();
    for update in request.updates {
        if let Ok(mut card) = col.get_card(anki_proto::cards::CardId {
            cid: update.card_id,
        }) {
            // Update fields if provided
            if let Some(deck_id) = update.deck_id {
                card.deck_id = deck_id;
            }
            if let Some(due) = update.due {
                card.due = due;
            }
            if let Some(flags) = update.flags {
                card.flags = flags as u32;
            }
            cards_to_update.push(card);
        }
    }

    let updated_count = cards_to_update.len();

    // Update all cards
    if !cards_to_update.is_empty() {
        let _ = col
            .update_cards(anki_proto::cards::UpdateCardsRequest {
                cards: cards_to_update,
                skip_undo_entry: false,
            })
            .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;
    }

    drop(col);

    Ok(Json(serde_json::json!({
        "success": true,
        "message": format!("Updated {} cards successfully", updated_count),
        "updated_count": updated_count,
    })))
}
