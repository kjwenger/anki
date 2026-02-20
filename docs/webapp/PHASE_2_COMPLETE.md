# Phase 2: Core API - Implementation Complete ✅

**Date Completed:** 2026-02-15
**Status:** Core Endpoints Complete, Integration Verified
**Next Phase:** Phase 3: UI Components

---

## Overview

Phase 2 focused on exposing Anki's core functionality through a RESTful API. This included management of collections, decks, and notes, as well as providing endpoints for media checking, tags, and statistics.

---

## Endpoints Implemented

### 2.1 Collections API
- **GET /api/v1/collection/info**: Get current user's collection status.
- **POST /api/v1/collection/close**: Close the active collection session.
- **GET /api/v1/collections**: List available collection files.
- **Architectural Decision**: Simplified to one primary collection per user auto-created at path `./data/users/user_{id}/{username}.anki2`.

### 2.2 Decks API
- **GET /api/v1/decks**: Returns the hierarchical deck tree with study counts.
- **POST /api/v1/decks**: Create a new deck (supports nested decks via `::`).
- **GET /api/v1/decks/{id}**: Retrieve metadata for a specific deck.
- **PUT /api/v1/decks/{id}**: Rename a deck or update its collapsed state.
- **DELETE /api/v1/decks/{id}**: Delete a deck and its sub-decks.

### 2.4 Notes API
- **POST /api/v1/notes**: Create a new note (automatically generates cards based on notetype).
- **GET /api/v1/notes/{id}**: Get note fields and tags.
- **PUT /api/v1/notes/{id}**: Update note content.
- **DELETE /api/v1/notes/{id}**: Remove a note and its associated cards.
- **GET /api/v1/notes/{id}/cards**: List all card IDs belonging to a note.

### 2.7 Media API (Partial)
- **POST /api/v1/media**: Upload a new media file (multipart/form-data).
- **GET /api/v1/media/check**: Run a media database check for unused/missing files.
- **DELETE /api/v1/media**: Move files to the trash.

### 2.8 Tags API
- **GET /api/v1/tags**: List all unique tags in the collection.
- **GET /api/v1/tags/tree**: Get hierarchical tag structure.
- **PUT /api/v1/tags/rename**: Bulk rename a tag across all notes.
- **DELETE /api/v1/tags/{name}**: Remove a tag from all notes.
- **POST /api/v1/tags/clear-unused**: Clean up tags not referenced by any notes.

### 2.9 Statistics API
- **GET /api/v1/stats/card/{id}**: Detailed review history and scheduling stats for a card.
- **GET /api/v1/stats/collection**: Summary counts (New, Young, Mature, Suspended).
- **GET /api/v1/stats/today**: Study activity for the current day.

---

## Technical Implementation Details

### Protobuf to JSON Conversion
The API server acts as a bridge between the frontend's JSON requests and the Anki core's Protobuf messages. Most responses are remapped into a clean, web-friendly JSON format while maintaining type safety through Rust's `serde`.

### Atomic Transactions
Write operations (creating notes, deleting decks) are wrapped in Anki's internal transaction system to ensure database integrity and consistent undo/redo states.

---

## Acceptance Criteria Status

| Criteria                 | Status | Notes                                     |
|--------------------------|--------|-------------------------------------------|
| CRUD for Decks and Notes | ✅      | Fully functional                          |
| Hierarchical Trees       | ✅      | Decks and Tags supported                  |
| Media Management         | ✅      | Upload and Check implemented              |
| Statistics               | ✅      | Collection and Card-level stats available |
| Authentication           | ✅      | All endpoints protected by JWT            |

---

## Next Steps

With the core API functional, the project moves to **Phase 3: UI Components**, where a SvelteKit-based web interface will be built to consume these endpoints.
