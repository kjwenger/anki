use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Extension;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;

use anki::services::StatsService;
use crate::auth::AuthUser;
use crate::error::Result;
use crate::error::WebAppError;
use crate::routes::AuthRouteState;

#[derive(Debug, Deserialize)]
pub struct GraphsQuery {
    pub search: Option<String>,
    pub days: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct CardStatsResponse {
    pub card_id: i64,
    pub note_id: i64,
    pub deck: String,
    pub added: i64,
    pub first_review: Option<i64>,
    pub latest_review: Option<i64>,
    pub due_date: Option<i64>,
    pub interval: u32,
    pub ease: u32,
    pub reviews: u32,
    pub lapses: u32,
    pub average_secs: f32,
    pub total_secs: f32,
    pub card_type: String,
    pub notetype: String,
}

#[derive(Debug, Serialize)]
pub struct TodayStatsResponse {
    pub answer_count: u32,
    pub answer_millis: u32,
    pub correct_count: u32,
    pub mature_correct: u32,
    pub mature_count: u32,
    pub learn_count: u32,
    pub review_count: u32,
    pub relearn_count: u32,
    pub early_review_count: u32,
}

#[derive(Debug, Serialize)]
pub struct CardCountsResponse {
    pub new_cards: u32,
    pub learn: u32,
    pub relearn: u32,
    pub young: u32,
    pub mature: u32,
    pub suspended: u32,
    pub buried: u32,
}

#[derive(Debug, Serialize)]
pub struct CollectionStatsResponse {
    pub today: TodayStatsResponse,
    pub card_counts: CardCountsResponse,
}

/// Get statistics for a specific card
pub async fn get_card_stats(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(card_id): Path<i64>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    // Get card stats
    let result = col
        .card_stats(anki::card::CardId(card_id))
        .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;

    drop(col);

    Ok(Json(CardStatsResponse {
        card_id: result.card_id,
        note_id: result.note_id,
        deck: result.deck,
        added: result.added,
        first_review: result.first_review,
        latest_review: result.latest_review,
        due_date: result.due_date,
        interval: result.interval,
        ease: result.ease,
        reviews: result.reviews,
        lapses: result.lapses,
        average_secs: result.average_secs,
        total_secs: result.total_secs,
        card_type: result.card_type,
        notetype: result.notetype,
    }))
}

/// Get graphs data for statistics
/// Note: This endpoint returns complex nested protobuf data.
/// For simpler statistics, use /api/v1/stats/collection or /api/v1/stats/today
pub async fn get_graphs(
    State(_state): State<AuthRouteState>,
    Extension(_auth_user): Extension<AuthUser>,
    Query(_query): Query<GraphsQuery>,
) -> Result<Json<serde_json::Value>> {
    // TODO: GraphsResponse doesn't implement Serialize
    // We would need to manually convert the protobuf structure to JSON
    // For now, return a not implemented message
    Err(WebAppError::not_implemented(
        "Graphs endpoint requires custom protobuf-to-JSON conversion",
    ))
}

/// Get collection-wide statistics
pub async fn get_collection_stats(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    Query(query): Query<GraphsQuery>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    // Get graphs data (includes today stats and card counts)
    let result = col
        .graphs(anki_proto::stats::GraphsRequest {
            search: query.search.unwrap_or_default(),
            days: query.days.unwrap_or(1),
        })
        .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;

    drop(col);

    // Extract today stats
    let today = result.today.unwrap_or_default();
    let card_counts = result
        .card_counts
        .and_then(|cc| cc.excluding_inactive)
        .unwrap_or_default();

    Ok(Json(CollectionStatsResponse {
        today: TodayStatsResponse {
            answer_count: today.answer_count,
            answer_millis: today.answer_millis,
            correct_count: today.correct_count,
            mature_correct: today.mature_correct,
            mature_count: today.mature_count,
            learn_count: today.learn_count,
            review_count: today.review_count,
            relearn_count: today.relearn_count,
            early_review_count: today.early_review_count,
        },
        card_counts: CardCountsResponse {
            new_cards: card_counts.new_cards,
            learn: card_counts.learn,
            relearn: card_counts.relearn,
            young: card_counts.young,
            mature: card_counts.mature,
            suspended: card_counts.suspended,
            buried: card_counts.buried,
        },
    }))
}

/// Get today's study statistics
pub async fn get_today_stats(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    // Get today's stats via graphs endpoint
    let result = col
        .graphs(anki_proto::stats::GraphsRequest {
            search: String::new(),
            days: 1,
        })
        .map_err(|e: anki::error::AnkiError| WebAppError::internal(&e.to_string()))?;

    drop(col);

    let today = result.today.unwrap_or_default();

    Ok(Json(TodayStatsResponse {
        answer_count: today.answer_count,
        answer_millis: today.answer_millis,
        correct_count: today.correct_count,
        mature_correct: today.mature_correct,
        mature_count: today.mature_count,
        learn_count: today.learn_count,
        review_count: today.review_count,
        relearn_count: today.relearn_count,
        early_review_count: today.early_review_count,
    }))
}
