# Web App vs Desktop App: Functionality Gap Analysis

Scope: Creating decks, adding notes/cards, and studying. Statistics are excluded.

---

## 1. Deck Management

### What the Web App Has

| Feature                         | API                         | UI                                                   |
|---------------------------------|-----------------------------|------------------------------------------------------|
| Create deck                     | `POST /api/v1/decks`        | Dialog with name input                               |
| List decks (flat with counts)   | `GET /api/v1/decks`         | `DeckTree` component showing new/learn/review counts |
| Rename deck                     | `PUT /api/v1/decks/{id}`    | Rename dialog                                        |
| Delete deck (with confirmation) | `DELETE /api/v1/decks/{id}` | Confirmation modal                                   |
| Nested decks via `::` separator | Supported in create/rename  | Displayed in tree                                    |

### What the Desktop App Adds

| Feature                         | Desktop Implementation                                                                                                            | Gap Severity |
|---------------------------------|-----------------------------------------------------------------------------------------------------------------------------------|--------------|
| **Drag-and-drop reparenting**   | `ReparentDecks` RPC; drag decks in tree to reorganize hierarchy                                                                   | Medium       |
| **Deck options/config**         | `DeckOptionsPage.svelte` (SvelteKit); configures new/review limits, FSRS params, learning steps, leech thresholds, audio settings | **High**     |
| **Filtered/custom study decks** | `GetOrCreateFilteredDeck`, `RebuildFilteredDeck`; build temporary decks from search queries with custom ordering                  | **High**     |
| **Custom study sessions**       | `CustomStudy` RPC; extend daily limits, review ahead, study by tag, preview new cards                                             | Medium       |
| **Collapse/expand state**       | `SetDeckCollapsed` RPC; remembers per-deck collapse state in browser vs reviewer separately                                       | Low          |
| **Deck descriptions**           | `Deck.Normal.description` / `markdown_description`; shown on overview screen                                                      | Low          |
| **Deck export**                 | Export deck as `.apkg` package for sharing                                                                                        | Medium       |

### Summary

The webapp covers the basic CRUD loop for decks. The biggest gaps are **deck options configuration** (learning steps, daily limits, FSRS parameters) and **filtered decks** (cram sessions, custom study). Without deck options, users are stuck with whatever defaults the backend assigns.

---

## 2. Adding Notes/Cards

### What the Web App Has

| Feature                        | API                                                   | UI                                                 |
|--------------------------------|-------------------------------------------------------|----------------------------------------------------|
| Create note with fields + tags | `POST /api/v1/notes`                                  | `editor/+page.svelte` with deck/notetype selectors |
| Select note type               | `GET /api/v1/notetypes`, `GET /api/v1/notetypes/{id}` | Dropdown listing available notetypes               |
| Select target deck             | `GET /api/v1/decks`                                   | Dropdown listing all decks                         |
| Edit fields                    | N/A (plain text)                                      | `FieldEditor.svelte` — a `<textarea>` per field    |
| Add tags                       | N/A                                                   | `TagInput.svelte` component                        |
| Read/update/delete notes       | `GET/PUT/DELETE /api/v1/notes/{id}`                   | Only via browse page                               |
| List notetypes with fields     | `GET /api/v1/notetypes/{id}`                          | Used to populate field editors                     |

### What the Desktop App Adds

| Feature                        | Desktop Implementation                                                                                                                           | Gap Severity               |
|--------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------|
| **Rich text editor (WYSIWYG)** | `NoteEditor.svelte` + `PlainTextInput.svelte` + rich text toolbar; bold, italic, underline, lists, colors, subscript/superscript, text alignment | **High**                   |
| **HTML editing mode**          | Toggle between rich text and raw HTML per field                                                                                                  | Medium                     |
| **Image support**              | Paste from clipboard, drag-and-drop, inline display; stored in media folder                                                                      | **High**                   |
| **Audio recording**            | Record audio inline with microphone; attach to fields                                                                                            | Medium                     |
| **Media attachments**          | Attach images, audio, video files to note fields                                                                                                 | **High**                   |
| **Cloze deletion helper**      | Toolbar button and keyboard shortcut to wrap selected text in `{{c1::...}}` syntax                                                               | **High** (for cloze users) |
| **LaTeX / MathJax**            | Live preview of math equations; MathJax overlay editor                                                                                           | Medium                     |
| **Duplicate detection**        | `NoteFieldsCheck` RPC; warns if first field matches existing note                                                                                | Medium                     |
| **Sticky fields**              | Fields retain values between additions for batch entry                                                                                           | Low                        |
| **Add history**                | Button to re-add previously created notes                                                                                                        | Low                        |
| **Field validation**           | Checks for empty fields, missing cloze numbers, etc.                                                                                             | Medium                     |
| **Image occlusion**            | Dedicated `image-occlusion` route; mask regions of image to create cards                                                                         | Medium                     |
| **Note type management**       | Full CRUD for notetypes: add/remove fields, edit card templates (HTML), configure CSS, change sort field                                         | **High**                   |
| **Change note type**           | `ChangeNotetype` dialog; migrate notes between types with field mapping                                                                          | Medium                     |
| **Keyboard shortcuts**         | Ctrl+Return to add, field-specific shortcuts, cloze shortcuts                                                                                    | Low                        |

### Summary

The webapp's editor is essentially a **plain text area per field**. The desktop app has a full rich text editor with media support, formatting toolbar, cloze helpers, duplicate detection, and LaTeX rendering. This is the **largest functional gap** — users cannot insert images, format text, or create cloze deletions through the web UI. Additionally, there is no way to create or modify note types; users are limited to whatever stock types exist in the collection.

---

## 3. Studying / Reviewing

### What the Web App Has

| Feature                            | API                                                       | UI                                        |
|------------------------------------|-----------------------------------------------------------|-------------------------------------------|
| Get next card (rendered HTML)      | `GET /api/v1/scheduler/decks/{id}/next`                   | `review/+page.svelte`                     |
| Answer card (Again/Hard/Good/Easy) | `POST /api/v1/scheduler/decks/{id}/cards/{cardId}/answer` | `AnswerButtons.svelte` — 4 buttons        |
| Show question → show answer flow   | N/A                                                       | "Show Answer" button + Space key          |
| Card counts (new/learning/review)  | Returned with each card response                          | `ReviewProgress.svelte`                   |
| Undo/redo                          | `POST /api/v1/scheduler/undo`, `/redo`                    | Buttons + Ctrl+Z/Ctrl+Shift+Z             |
| Keyboard shortcuts                 | N/A                                                       | Space/Enter to reveal, 1-4 to rate        |
| Completion screen                  | N/A                                                       | "Study Complete!" with back-to-decks link |
| CSS injection for card styling     | Card CSS applied via `<style>` element                    | `CardDisplay.svelte`                      |

### What the Desktop App Adds

| Feature                          | Desktop Implementation                                                                                  | Gap Severity |
|----------------------------------|---------------------------------------------------------------------------------------------------------|--------------|
| **Overview screen**              | `overview.py`; shows deck name, description, counts, study button, config access before entering review | Medium       |
| **Next interval preview**        | `DescribeNextStates` RPC; shows "1d", "4d", "1.2mo" on each answer button                               | **High**     |
| **Time tracking**                | `milliseconds_taken` field in `CardAnswer`; tracks how long user spent on each card                     | Medium       |
| **Audio replay**                 | Auto-play audio on question/answer; replay button; configurable play order                              | Medium       |
| **Type-in-the-answer**           | Compare typed answer to correct answer with diff highlighting                                           | Medium       |
| **Edit card during review**      | Open editor inline to fix typos without leaving review                                                  | Medium       |
| **Flag cards**                   | Mark cards with colored flags (1-7) during review for later attention                                   | Low          |
| **Suspend/bury during review**   | Right-click or shortcut to suspend or bury the current card                                             | Medium       |
| **Card info during review**      | View scheduling details, review history for current card                                                | Low          |
| **Filtered deck study**          | Study from custom search queries with configurable ordering                                             | **High**     |
| **Custom study**                 | Extend limits, review ahead, study by tag, preview new                                                  | Medium       |
| **Per-deck study limits**        | Respect and display new/review limits from deck config                                                  | **High**     |
| **FSRS scheduling display**      | Show desired retention, stability, difficulty metrics                                                   | Low          |
| **Deck-specific card selection** | Webapp ignores `deck_id` param — fetches from entire collection queue                                   | **High**     |
| **Congrats screen with options** | Shows stats, offers custom study, filtered deck creation                                                | Low          |
| **Timer display**                | Shows elapsed study time                                                                                | Low          |

### Critical Bug: Deck Filtering

Looking at the scheduler endpoint (`rslib/webapp/src/routes/scheduler.rs:56`), the `_deck_id` path parameter is **ignored** — the code calls `col.get_queued_cards(1, false)` without filtering by deck. This means studying always pulls from the entire collection queue rather than the selected deck. This is a significant behavioral deviation from desktop Anki.

### Summary

The webapp has a functional basic review loop. The biggest gaps are: no **interval preview on buttons** (users study blind without knowing what each rating means in terms of next review), no **deck-scoped study** (the deck_id parameter is ignored), and no **deck configuration** feeding into study limits. The desktop reviewer is also significantly richer in terms of in-review actions (edit, flag, suspend, bury) and media playback.

---

## Cross-Cutting Gaps

| Area                    | Gap                                                                                                                                                             | Severity                    |
|-------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------|
| **Import/Export**       | Desktop supports `.apkg`, `.colpkg`, CSV import/export. Webapp has no import UI.                                                                                | **High**                    |
| **Sync**                | Desktop syncs with AnkiWeb. Webapp is fully isolated.                                                                                                           | **High** (for multi-device) |
| **Search/Browse**       | Webapp has `browse/+page.svelte` and search API but limited editing capabilities compared to desktop's full browser with inline editing, sort, bulk operations. | Medium                      |
| **Undo granularity**    | Desktop tracks per-operation undo with `OpChanges` that selectively refresh UI. Webapp undo is a simple toggle.                                                 | Low                         |
| **Keyboard navigation** | Desktop has extensive keyboard shortcuts across all views. Webapp only has reviewer shortcuts.                                                                  | Low                         |
| **Add-on ecosystem**    | Desktop supports Python add-ons via hooks. Webapp has no extension mechanism.                                                                                   | Low (for MVP)               |

---

## Priority Ranking for Closing Gaps

### Must-Have (Core loop is broken or severely limited without these)

1. **Fix deck-scoped study** — `scheduler.rs` ignores `deck_id`; cards come from entire collection
2. **Rich text editor with image support** — plain `<textarea>` makes card creation impractical for most real-world use
3. **Interval preview on answer buttons** — users need to see what each rating does ("1min", "10min", "1d", "4d")
4. **Deck options configuration** — without this, users cannot set daily limits, learning steps, or FSRS parameters

### Should-Have (Important for practical daily use)

5. **Cloze deletion support** — cloze is one of the most popular note types; needs at minimum a helper to insert `{{c1::...}}`
6. **Duplicate detection** — prevents wasted effort creating redundant cards
7. **Filtered decks / custom study** — core desktop feature for exam prep and targeted review
8. **Media upload for note fields** — images are essential for many study domains (anatomy, geography, etc.)
9. **Import `.apkg` packages** — lets users bring in shared decks, which is how most people start with Anki

### Nice-to-Have (Polish and parity)

10. Note type management (create/edit fields and templates)
11. Overview screen before study
12. Time tracking per card
13. Audio playback during review
14. Flag/suspend/bury during review
15. Drag-and-drop deck rearrangement
