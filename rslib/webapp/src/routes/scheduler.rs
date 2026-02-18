use anki::scheduler::answering::CardAnswer;
use anki::scheduler::answering::Rating;
use anki::services::CardsService;
use anki::timestamp::TimestampMillis;
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

#[derive(Debug, Serialize)]
pub struct QueuedCardResponse {
    pub card_id: i64,
    pub question_html: String,
    pub answer_html: String,
    pub css: String,
    pub counts: StudyCounts,
    pub flags: u8,
}

#[derive(Debug, Serialize)]
pub struct StudyCounts {
    pub new: usize,
    pub learning: usize,
    pub review: usize,
}

#[derive(Debug, Deserialize)]
pub struct AnswerCardRequest {
    pub rating: u8, // 0=Again, 1=Hard, 2=Good, 3=Easy
    #[serde(default)]
    pub milliseconds_taken: u32,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub success: bool,
    pub message: String,
}

/// Get the next card to study from a specific deck
pub async fn get_next_card(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(deck_id): Path<i64>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    // Set the current deck to scope the scheduler to this deck
    col.set_current_deck(deck_id.into())
        .map_err(|e| WebAppError::internal(&e.to_string()))?;

    let queued_cards = col
        .get_queued_cards(1, false)
        .map_err(|e| WebAppError::internal(&e.to_string()))?;

    if let Some(queued_card) = queued_cards.cards.first() {
        let card_id = queued_card.card.id();

        // Get the full card to access flags
        let full_card = <anki::collection::Collection as CardsService>::get_card(
            &mut *col,
            anki_proto::cards::CardId { cid: card_id.0 },
        )
        .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;

        // Render the card HTML
        let rendered = col
            .render_existing_card(card_id, false, false)
            .map_err(|e| WebAppError::internal(&e.to_string()))?;

        let response = QueuedCardResponse {
            card_id: card_id.0,
            question_html: rendered.question().into_owned(),
            answer_html: rendered.answer().into_owned(),
            css: rendered.css.clone(),
            counts: StudyCounts {
                new: queued_cards.new_count,
                learning: queued_cards.learning_count,
                review: queued_cards.review_count,
            },
            flags: full_card.flags as u8,
        };

        drop(col);
        Ok(Json(serde_json::json!({
            "card": response,
            "finished": false,
        })))
    } else {
        // No more cards
        drop(col);
        Ok(Json(serde_json::json!({
            "card": null,
            "finished": true,
        })))
    }
}

/// Answer a card and get the next one
pub async fn answer_card(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    Path((_deck_id, card_id)): Path<(i64, i64)>,
    Json(request): Json<AnswerCardRequest>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    // Get the queued card to get its states
    let queued_cards = col
        .get_queued_cards(1, false)
        .map_err(|e| WebAppError::internal(&e.to_string()))?;

    if let Some(queued) = queued_cards.cards.first() {
        let queued_card_id = queued.card.id();

        if queued_card_id.0 != card_id {
            drop(col);
            return Err(WebAppError::bad_request(
                "Card is not the current card in queue",
            ));
        }

        let rating = match request.rating {
            0 => Rating::Again,
            1 => Rating::Hard,
            2 => Rating::Good,
            3 => Rating::Easy,
            _ => {
                drop(col);
                return Err(WebAppError::bad_request(
                    "Invalid rating value. Must be 0-3.",
                ));
            }
        };

        // Pick the new state based on the rating
        let new_state = match rating {
            Rating::Again => queued.states.again,
            Rating::Hard => queued.states.hard,
            Rating::Good => queued.states.good,
            Rating::Easy => queued.states.easy,
        };

        let mut answer = CardAnswer {
            card_id: queued_card_id,
            current_state: queued.states.current,
            new_state,
            rating,
            answered_at: TimestampMillis::now(),
            milliseconds_taken: request.milliseconds_taken,
            custom_data: None,
            from_queue: true,
        };

        col.answer_card(&mut answer)
            .map_err(|e| WebAppError::internal(&e.to_string()))?;

        drop(col);

        Ok(Json(MessageResponse {
            success: true,
            message: "Card answered successfully".to_string(),
        }))
    } else {
        drop(col);
        Err(WebAppError::not_found("No card in queue"))
    }
}

/// Get next interval descriptions for a card's answer buttons
pub async fn get_next_states(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    Path((_deck_id, card_id)): Path<(i64, i64)>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    // Get scheduling states for the card
    let states = col
        .get_scheduling_states(card_id.into())
        .map_err(|e| WebAppError::internal(&e.to_string()))?;

    // Get human-readable interval descriptions
    let descriptions = col
        .describe_next_states(&states)
        .map_err(|e| WebAppError::internal(&e.to_string()))?;

    drop(col);

    // descriptions is a Vec<String> with 4 entries: [Again, Hard, Good, Easy]
    #[derive(Serialize)]
    struct NextStatesResponse {
        again: String,
        hard: String,
        good: String,
        easy: String,
    }

    let response = NextStatesResponse {
        again: descriptions.get(0).cloned().unwrap_or_default(),
        hard: descriptions.get(1).cloned().unwrap_or_default(),
        good: descriptions.get(2).cloned().unwrap_or_default(),
        easy: descriptions.get(3).cloned().unwrap_or_default(),
    };

    Ok(Json(response))
}

/// Get study counts for a specific deck
pub async fn get_deck_counts(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(deck_id): Path<i64>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    col.set_current_deck(deck_id.into())
        .map_err(|e| WebAppError::internal(&e.to_string()))?;

    let queued_cards = col
        .get_queued_cards(0, false)
        .map_err(|e| WebAppError::internal(&e.to_string()))?;

    drop(col);

    Ok(Json(StudyCounts {
        new: queued_cards.new_count,
        learning: queued_cards.learning_count,
        review: queued_cards.review_count,
    }))
}

/// Undo the last operation
pub async fn undo(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    match col.undo() {
        Ok(_) => {
            drop(col);
            Ok(Json(MessageResponse {
                success: true,
                message: "Action undone successfully".to_string(),
            }))
        }
        Err(e) => {
            drop(col);
            if e.to_string().is_empty() || matches!(e, anki::error::AnkiError::UndoEmpty) {
                Err(WebAppError::bad_request("Nothing to undo"))
            } else {
                Err(WebAppError::internal(&e.to_string()))
            }
        }
    }
}

/// Redo the last undone operation
pub async fn redo(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    match col.redo() {
        Ok(_) => {
            drop(col);
            Ok(Json(MessageResponse {
                success: true,
                message: "Action redone successfully".to_string(),
            }))
        }
        Err(e) => {
            drop(col);
            if e.to_string().is_empty() || matches!(e, anki::error::AnkiError::UndoEmpty) {
                Err(WebAppError::bad_request("Nothing to redo"))
            } else {
                Err(WebAppError::internal(&e.to_string()))
            }
        }
    }
}
