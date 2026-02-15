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

#[derive(Debug, Serialize)]
pub struct QueuedCardResponse {
    pub card_id: i64,
    pub question_html: String,
    pub answer_html: String,
    pub counts: StudyCounts,
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
    Path(_deck_id): Path<i64>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    // Get next card(s) from the queue using the service
    let queued_cards = col
        .get_queued_cards(anki_proto::scheduler::GetQueuedCardsRequest {
            fetch_limit: 1,
            intraday_learning_only: false,
        })
        .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;

    if let Some(queued_card) = queued_cards.cards.first() {
        let card_id = queued_card.card.as_ref().map(|c| c.id).unwrap_or(0);
        
        // Render the card HTML using the protobuf card
        let question = col
            .render_card(anki_proto::scheduler::RenderCardRequest {
                card_id,
                browser: false,
                question_side: true,
            })
            .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;

        let answer = col
            .render_card(anki_proto::scheduler::RenderCardRequest {
                card_id,
                browser: false,
                question_side: false,
            })
            .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;

        let response = QueuedCardResponse {
            card_id,
            question_html: question.question_and_style,
            answer_html: answer.answer_and_style,
            counts: StudyCounts {
                new: queued_cards.new_count as usize,
                learning: queued_cards.learning_count as usize,
                review: queued_cards.review_count as usize,
            },
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
        .get_queued_cards(anki_proto::scheduler::GetQueuedCardsRequest {
            fetch_limit: 1,
            intraday_learning_only: false,
        })
        .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;

    if let Some(queued) = queued_cards.cards.first() {
        let queued_card_id = queued.card.as_ref().map(|c| c.id).unwrap_or(0);
        
        if queued_card_id != card_id {
            drop(col);
            return Err(WebAppError::bad_request(
                "Card is not the current card in queue",
            ));
        }

        // Validate rating
        if request.rating > 3 {
            drop(col);
            return Err(WebAppError::bad_request("Invalid rating value. Must be 0-3."));
        }

        // Answer the card using the service
        let _ = col
            .answer_card(anki_proto::scheduler::CardAnswer {
                card_id,
                current_state: queued.states.clone(),
                new_state: queued.states.clone(), // Will be calculated by next_states
                rating: request.rating as i32,
                answered_at_millis: 0, // Use current time
                milliseconds_taken: 0, // TODO: track this in UI
            })
            .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;

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

/// Get study counts for a specific deck
pub async fn get_deck_counts(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(_deck_id): Path<i64>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    // Get current counts
    let queued_cards = col
        .get_queued_cards(anki_proto::scheduler::GetQueuedCardsRequest {
            fetch_limit: 0, // Just get counts
            intraday_learning_only: false,
        })
        .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;

    drop(col);

    Ok(Json(StudyCounts {
        new: queued_cards.new_count as usize,
        learning: queued_cards.learning_count as usize,
        review: queued_cards.review_count as usize,
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

    let _ = col
        .undo()
        .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;

    drop(col);

    Ok(Json(MessageResponse {
        success: true,
        message: "Action undone successfully".to_string(),
    }))
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

    let _ = col
        .redo()
        .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;

    drop(col);

    Ok(Json(MessageResponse {
        success: true,
        message: "Action redone successfully".to_string(),
    }))
}
