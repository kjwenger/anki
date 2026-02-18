// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use std::collections::HashMap;

use anki::decks::DeckId;
use anki::notes::NoteId;
use anki::notetype::NotetypeId;
use anki::services::CardsService;
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
pub struct BrowseRequest {
    pub ids: Vec<i64>,
}

#[derive(Debug, Serialize)]
pub struct CardBrowseRow {
    pub card_id: i64,
    pub sort_field: String,
    pub card_type: String,
    pub due: String,
    pub deck: String,
}

#[derive(Debug, Serialize)]
pub struct NoteBrowseRow {
    pub note_id: i64,
    pub sort_field: String,
    pub notetype: String,
    pub cards: usize,
    pub tags: String,
}

/// Batch endpoint: given a list of card IDs, return browse-table rows with
/// Sort Field, Card Type, Due, and Deck — matching the desktop app's card browser.
pub async fn browse_cards(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(request): Json<BrowseRequest>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    let timing = col
        .timing_today()
        .map_err(|e| WebAppError::internal(&e.to_string()))?;
    let days_elapsed = timing.days_elapsed as i32;

    // Phase 1: collect raw data for each card using the proto-based public API.
    struct RawCard {
        card_id: i64,
        deck_id: i64,
        notetype_id: i64,
        template_idx: u32,
        queue: i32,
        due: i32,
        sort_field: String,
    }

    let mut raw: Vec<RawCard> = Vec::with_capacity(request.ids.len());
    let mut notetype_ids: std::collections::HashSet<i64> = Default::default();
    let mut deck_ids: std::collections::HashSet<i64> = Default::default();

    for card_id in &request.ids {
        // get_card via CardsService returns anki_proto::cards::Card with public fields
        let card = match col.get_card(anki_proto::cards::CardId { cid: *card_id }) {
            Ok(c) => c,
            Err(_) => continue,
        };
        // storage.get_note is &self, safe to call after get_card returns
        let note = match col.storage.get_note(NoteId(card.note_id)) {
            Ok(Some(n)) => n,
            _ => continue,
        };
        let sort_field = note.fields().first().cloned().unwrap_or_default();
        let notetype_id = note.notetype_id.0;
        let deck_id = card.deck_id;
        notetype_ids.insert(notetype_id);
        deck_ids.insert(deck_id);
        raw.push(RawCard {
            card_id: *card_id,
            deck_id,
            notetype_id,
            template_idx: card.template_idx,
            queue: card.queue,
            due: card.due,
            sort_field,
        });
    }

    // Phase 2: batch-fetch unique notetypes and decks.
    let mut notetype_templates: HashMap<i64, Vec<String>> = HashMap::new();
    for ntid in notetype_ids {
        if let Ok(Some(nt)) = col.get_notetype(NotetypeId(ntid)) {
            notetype_templates.insert(
                ntid,
                nt.templates.iter().map(|t| t.name.clone()).collect(),
            );
        }
    }

    let mut deck_names: HashMap<i64, String> = HashMap::new();
    for did in deck_ids {
        if let Ok(Some(deck)) = col.get_deck(DeckId(did)) {
            deck_names.insert(did, deck.name.human_name());
        }
    }

    drop(col);

    // Phase 3: assemble rows.
    let rows: Vec<CardBrowseRow> = raw
        .into_iter()
        .map(|r| {
            let card_type = notetype_templates
                .get(&r.notetype_id)
                .and_then(|ts| ts.get(r.template_idx as usize))
                .cloned()
                .unwrap_or_default();
            let deck = deck_names.get(&r.deck_id).cloned().unwrap_or_default();
            let due = format_due(r.queue, r.due, days_elapsed);
            CardBrowseRow {
                card_id: r.card_id,
                sort_field: r.sort_field,
                card_type,
                due,
                deck,
            }
        })
        .collect();

    Ok(Json(serde_json::json!({ "rows": rows })))
}

/// Batch endpoint: given a list of note IDs, return browse-table rows with
/// Sort Field, Note Type, Cards count, and Tags — matching the desktop app's note browser.
pub async fn browse_notes(
    State(state): State<AuthRouteState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(request): Json<BrowseRequest>,
) -> Result<impl IntoResponse> {
    let backend = state
        .backend_manager
        .get_or_create_backend(auth_user.user_id, &auth_user.username)?;

    let mut col = backend.lock().unwrap();

    // Phase 1: collect raw data.
    struct RawNote {
        note_id: i64,
        notetype_id: i64,
        sort_field: String,
        card_count: usize,
        tags: String,
    }

    let mut raw: Vec<RawNote> = Vec::with_capacity(request.ids.len());
    let mut notetype_ids: std::collections::HashSet<i64> = Default::default();

    for note_id in &request.ids {
        let note = match col.storage.get_note(NoteId(*note_id)) {
            Ok(Some(n)) => n,
            _ => continue,
        };
        let sort_field = note.fields().first().cloned().unwrap_or_default();
        let tags = note.tags.join(" ");
        let notetype_id = note.notetype_id.0;
        let card_count = col
            .storage
            .all_cards_of_note(NoteId(*note_id))
            .map(|v| v.len())
            .unwrap_or(0);
        notetype_ids.insert(notetype_id);
        raw.push(RawNote {
            note_id: *note_id,
            notetype_id,
            sort_field,
            card_count,
            tags,
        });
    }

    // Phase 2: batch-fetch unique notetypes.
    let mut notetype_names: HashMap<i64, String> = HashMap::new();
    for ntid in notetype_ids {
        if let Ok(Some(nt)) = col.get_notetype(NotetypeId(ntid)) {
            notetype_names.insert(ntid, nt.name.clone());
        }
    }

    drop(col);

    // Phase 3: assemble rows.
    let rows: Vec<NoteBrowseRow> = raw
        .into_iter()
        .map(|r| {
            let notetype = notetype_names
                .get(&r.notetype_id)
                .cloned()
                .unwrap_or_default();
            NoteBrowseRow {
                note_id: r.note_id,
                sort_field: r.sort_field,
                notetype,
                cards: r.card_count,
                tags: r.tags,
            }
        })
        .collect();

    Ok(Json(serde_json::json!({ "rows": rows })))
}

/// Format the due date into a human-readable string matching the desktop browser.
/// queue values: 0=New, 1=Learn, 2=Review, 3=DayLearn, 4=PreviewRepeat,
///               -1=Suspended, -2=SchedBuried, -3=UserBuried
fn format_due(queue: i32, due: i32, days_elapsed: i32) -> String {
    match queue {
        0 => format!("#{due}"),          // New: position in new queue
        1 | 4 => "Learning".to_string(), // Learn / PreviewRepeat
        2 | 3 => {
            // Review / DayLearn: due is an absolute day number
            let rel = due - days_elapsed;
            match rel {
                i32::MIN..=-1 => format!("{} day{} overdue", -rel, if -rel == 1 { "" } else { "s" }),
                0 => "Today".to_string(),
                1 => "In 1 day".to_string(),
                _ => format!("In {rel} days"),
            }
        }
        -1 => "Suspended".to_string(),
        _ => "Buried".to_string(), // -2 SchedBuried, -3 UserBuried
    }
}
