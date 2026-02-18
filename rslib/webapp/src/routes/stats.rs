use anki::services::SearchService;
use anki::services::StatsService;
use axum::extract::Path;
use axum::extract::Query;
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

/// Count cards matching an Anki search query.
fn count_cards(
    col: &mut anki::collection::Collection,
    query: &str,
) -> anki::error::Result<u32> {
    let result = <anki::collection::Collection as SearchService>::search_cards(
        col,
        anki_proto::search::SearchRequest {
            search: query.to_string(),
            order: None,
        },
    )?;
    Ok(result.ids.len() as u32)
}

/// Count notes matching an Anki search query.
fn count_notes(
    col: &mut anki::collection::Collection,
    query: &str,
) -> anki::error::Result<u32> {
    let result = <anki::collection::Collection as SearchService>::search_notes(
        col,
        anki_proto::search::SearchRequest {
            search: query.to_string(),
            order: None,
        },
    )?;
    Ok(result.ids.len() as u32)
}

/// Get collection-wide statistics
pub async fn get_collection_stats(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    // Each search is independent — no heavyweight graphs() call or temp-table
    // interactions that could silently return zeros.
    //
    // Query notes:
    //   is:new               → c.type = 0  (CardType::New, any queue)
    //   -is:suspended        → exclude queue = -1
    //   -is:buried           → exclude queue = -2 / -3
    //   is:review            → c.type in (2, 3)  (Review + Relearn)
    //   prop:ivl<21          → interval < 21 days  (young)
    //   prop:ivl>=21         → interval ≥ 21 days  (mature)

    let total_cards = count_cards(&mut *col, "")
        .map_err(|e| WebAppError::internal(&e.to_string()))?;

    let total_notes = count_notes(&mut *col, "")
        .map_err(|e| WebAppError::internal(&e.to_string()))?;

    let new_cards = count_cards(&mut *col, "is:new -is:suspended -is:buried")
        .map_err(|e| WebAppError::internal(&e.to_string()))?;

    let young_cards =
        count_cards(&mut *col, "is:review prop:ivl<21 -is:suspended -is:buried")
            .map_err(|e| WebAppError::internal(&e.to_string()))?;

    let mature_cards =
        count_cards(&mut *col, "is:review prop:ivl>=21 -is:suspended -is:buried")
            .map_err(|e| WebAppError::internal(&e.to_string()))?;

    let suspended_cards = count_cards(&mut *col, "is:suspended")
        .map_err(|e| WebAppError::internal(&e.to_string()))?;

    let buried_cards = count_cards(&mut *col, "is:buried")
        .map_err(|e| WebAppError::internal(&e.to_string()))?;

    drop(col);

    Ok(Json(serde_json::json!({
        "total_cards": total_cards,
        "new_cards": new_cards,
        "young_cards": young_cards,
        "mature_cards": mature_cards,
        "suspended_cards": suspended_cards,
        "buried_cards": buried_cards,
        "total_notes": total_notes,
    })))
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
