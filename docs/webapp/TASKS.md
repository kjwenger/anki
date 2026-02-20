# Anki Web App - Implementation Tasks

This document breaks down the implementation into manageable tasks with priorities and time estimates.

## Table of Contents

- [Phase 1: Foundation](#phase-1-foundation-2-weeks) ✅ — Core infrastructure and authentication
  - [1.1 Project Structure Setup](#11-project-structure-setup)
  - [1.2 Database Schema for Users](#12-database-schema-for-users-)
  - [1.3 Authentication System](#13-authentication-system-)
  - [1.4 Session Management](#14-session-management-)
  - [1.5 Configuration System](#15-configuration-system-)
  - [1.6 Error Handling](#16-error-handling-)
- [Phase 2: Core API](#phase-2-core-api-2-weeks) ✅ — Essential REST endpoints
  - [2.1 Collections API](#21-collections-api-)
  - [2.2 Decks API](#22-decks-api-)
  - [2.3 Scheduler API](#23-scheduler-api-)
  - [2.4 Notes API](#24-notes-api-)
  - [2.5 Cards API](#25-cards-api-)
  - [2.6 Search API](#26-search-api-)
  - [2.7 Media API](#27-media-api--partial)
  - [2.8 Tags API](#28-tags-api-)
  - [2.9 Statistics API](#29-statistics-api-)
- [Phase 3: UI Components](#phase-3-ui-components-3-weeks) ✅ — Web interface pages
  - [3.1 Authentication UI](#31-authentication-ui-)
  - [3.2 Collection Manager UI](#32-collection-manager-ui-)
  - [3.3 Deck Browser UI](#33-deck-browser-ui-)
  - [3.4 Reviewer UI](#34-reviewer-ui-)
  - [3.5 Editor UI](#35-editor-ui)
  - [3.6 Card Browser UI](#36-card-browser-ui)
  - [3.7 Statistics UI](#37-statistics-ui)
  - [3.8 Settings UI](#38-settings-ui)
  - [3.9 Navigation & Layout](#39-navigation--layout)
- [Phase 4: Desktop Parity — Quick Wins](#phase-4-desktop-parity--quick-wins) — Low-hanging-fruit gap closures
  - [4.1 Critical Bug Fixes](#41-critical-bug-fixes-) ✅
  - [4.2 Interval Preview on Answer Buttons](#42-interval-preview-on-answer-buttons-) ✅ — complete
  - [4.3 Time Tracking Per Card](#43-time-tracking-per-card-) ✅
  - [4.4 Flag / Suspend / Bury During Review](#44-flag--suspend--bury-during-review-) ✅
  - [4.5 Cloze Deletion Toolbar Helper](#45-cloze-deletion-toolbar-helper-) ✅
  - [4.6 Sticky Fields in Editor](#46-sticky-fields-in-editor-) ✅
  - [4.7 Duplicate Detection in Editor](#47-duplicate-detection-in-editor-) ✅
  - [4.8 Deck Collapse / Expand State](#48-deck-collapse--expand-state-) ✅
  - [4.9 Overview Screen Before Study](#49-overview-screen-before-study-) ✅
  - [4.10 Audio Playback During Review](#410-audio-playback-during-review-) ✅
  - [4.11 Keyboard Shortcuts in Browse and Editor](#411-keyboard-shortcuts-in-browse-and-editor-) ✅
  - [4.12 APKG Import](#412-apkg-import) ✅
  - [4.13 APKG Export](#413-apkg-export)
- [Phase 5: Polish & Testing](#phase-5-polish--testing) — Testing, documentation, deployment
  - [5.1 API Testing](#51-api-testing-) ✅
  - [5.2 UI Testing](#52-ui-testing)
  - [5.3 Documentation](#53-documentation)
  - [5.4 Security Audit](#54-security-audit)
  - [5.5 Performance Optimization](#55-performance-optimization)
  - [5.6 Deployment Packaging](#56-deployment-packaging)
- [Optional Enhancements](#optional-enhancements-future)
- [Timeline Summary](#timeline-summary)
- [Risk Assessment](#risk-assessment)
- [Success Criteria](#success-criteria)

---

## Phase 1: Foundation (2 weeks)

### 1.1 Project Structure Setup

**Priority**: P0 (Critical)\
**Estimate**: 2 days\
**Dependencies**: None

- [ ] Create `rslib/webapp/` directory structure
- [ ] Set up Cargo.toml for webapp module
- [ ] Add webapp to workspace members
- [ ] Create basic Axum server scaffolding
- [ ] Set up SvelteKit routes in `ts/routes/webapp/`
- [ ] Configure build system integration

**Acceptance Criteria**:

- Server compiles and runs on localhost
- SvelteKit app accessible in browser
- `./check` passes with new code

---

### 1.2 Database Schema for Users ✅

**Priority**: P0\
**Estimate**: 1 day\
**Dependencies**: 1.1\
**Status**: Complete

- [x] Create SQLite schema for users table
- [x] Create sessions table
- [x] Implement migration system
- [x] Add database initialization code
- [x] Write database helper functions

**Files Created**:

- `rslib/webapp/src/db/schema.sql`
- `rslib/webapp/src/db/mod.rs`
- `rslib/webapp/src/db/users.rs`
- `rslib/webapp/src/db/sessions.rs`

**Acceptance Criteria** (All Met):

- ✅ Database creates on first run
- ✅ Users can be added/queried
- ✅ Sessions can be stored/retrieved

---

### 1.3 Authentication System ✅

**Priority**: P0\
**Estimate**: 3 days\
**Dependencies**: 1.2\
**Status**: Complete

- [x] Implement password hashing (argon2)
- [x] Create JWT token generation/validation
- [x] Build registration endpoint
- [x] Build login endpoint
- [x] Build logout endpoint
- [x] Create auth middleware
- [x] Add token refresh mechanism

**Files Created**:

- `rslib/webapp/src/auth/mod.rs`
- `rslib/webapp/src/auth/jwt.rs`
- `rslib/webapp/src/auth/password.rs`
- `rslib/webapp/src/auth/middleware.rs`
- `rslib/webapp/src/routes/mod.rs`
- `rslib/webapp/src/routes/auth.rs`

**Acceptance Criteria** (All Met):

- ✅ User can register with username/password
- ✅ User can login and receive JWT
- ✅ Protected routes require valid JWT
- ✅ Password is securely hashed with Argon2
- ✅ Sessions are tracked and can be invalidated
- Password is securely hashed

---

### 1.4 Session Management ✅

**Priority**: P0\
**Estimate**: 2 days\
**Dependencies**: 1.3\
**Status**: Complete

- [x] Implement session store (completed in 1.2)
- [x] Create session middleware (completed in 1.3)
- [x] Add per-user Backend instance management
- [x] Implement collection opening/closing
- [x] Add session cleanup/timeout (completed in 1.2)

**Files Created**:

- `rslib/webapp/src/session/mod.rs`
- `rslib/webapp/src/session/backend.rs`
- `rslib/webapp/src/routes/collection.rs`

**Acceptance Criteria** (All Met):

- ✅ Each user gets isolated Backend instance
- ✅ Sessions timeout after inactivity (database-based)
- ✅ Collections properly closed on logout
- ✅ Concurrent requests handled correctly (Mutex-based locking)
- ✅ Backend instances cached and reused per user
- ✅ Collection files created in user-specific directories

---

### 1.5 Configuration System ✅

**Priority**: P1\
**Estimate**: 1 day\
**Dependencies**: 1.1\
**Status**: Complete

- [x] Create configuration struct
- [x] Implement TOML config loading
- [x] Add environment variable overrides
- [x] Create default configuration
- [x] Document configuration options

**Files Created**:

- `config/server.toml.example` - Example configuration file
- `config/README.md` - Configuration documentation

**Files Enhanced**:

- `rslib/webapp/src/config.rs` - Added TOML file loading and tests

**Acceptance Criteria** (All Met):

- ✅ Server loads config from file (config/server.toml)
- ✅ ENV vars override config values (highest priority)
- ✅ Missing config uses sensible defaults
- ✅ Priority: ENV > File > Defaults
- ✅ Comprehensive documentation
- ✅ 4 unit tests passing

---

### 1.6 Error Handling ✅

**Priority**: P1\
**Estimate**: 1 day\
**Dependencies**: 1.1\
**Status**: Complete

- [x] Create error types for webapp
- [x] Implement error to HTTP status mapping
- [x] Add error response formatting
- [x] Create custom error middleware (IntoResponse trait)
- [x] Add error logging

**Files Enhanced**:

- `rslib/webapp/src/error.rs` - Added logging and comprehensive tests

**Acceptance Criteria** (All Met):

- ✅ Consistent error response format (JSON with success: false)
- ✅ Appropriate HTTP status codes (500, 400, 401, 404, 409, 403)
- ✅ Errors logged with context (error/warn/debug levels)
- ✅ No sensitive info in error messages (internal errors sanitized)
- ✅ 9 comprehensive error tests passing
- ✅ Helper constructors for each error type
- ✅ anyhow::Error conversion support

---

## Phase 2: Core API (2 weeks)

### 2.1 Collections API ✅

**Priority**: P0\
**Estimate**: 2 days\
**Dependencies**: 1.4\
**Status**: Complete (Simplified Architecture)

**Architectural Decision:**\
Single collection per user (standard web app pattern) instead of multiple collections.
Collection management handled through backend manager (Task 1.4).

Endpoints Implemented:

- [x] GET /api/v1/collection/info (get current user's collection)
- [x] POST /api/v1/collection/close (close collection)

**Note:** In our web app architecture:

- Each user has ONE collection (auto-created on first access)
- Collection lifecycle tied to user session
- Collection path: `./data/users/user_{id}/{username}.anki2`
- No need for collection CRUD - it's automatic

**Files Enhanced**:

- `rslib/webapp/src/routes/collection.rs` - Collection endpoints

**Acceptance Criteria** (Met):

- ✅ Collections isolated per user (Task 1.4)
- ✅ Auto-creation on first access (Task 1.4)
- ✅ Proper cleanup on logout (Task 1.4)
- ✅ Collection info endpoint
- ✅ Collection close endpoint

---

### 2.2 Decks API ✅

**Priority**: P0\
**Estimate**: 2 days\
**Dependencies**: 2.1\
**Status**: Core Complete

Endpoints Implemented:

- [x] GET /api/v1/decks (deck tree)
- [x] POST /api/v1/decks (create)
- [x] GET /api/v1/decks/{id} (get)
- [x] DELETE /api/v1/decks/{id} (delete)
- [x] PUT /api/v1/decks/{id} (update)
- [ ] POST /api/v1/decks/{id}/rename - future
- [ ] GET /api/v1/decks/{id}/stats - future
- [ ] POST /api/v1/decks/{id}/set-current - future
- [ ] GET /api/v1/decks/{id}/config - future
- [ ] PUT /api/v1/decks/{id}/config - future

**Files Created**:

- `rslib/webapp/src/routes/decks.rs` - Deck route handlers

**Acceptance Criteria** (Core Met):

- ✅ Deck tree returns correct hierarchy
- ✅ Deck CRUD operations work (create, read, delete)
- ✅ Decks properly isolated per user
- ⏭️ Statistics (deferred - Task 2.3)
- ⏭️ Config changes (deferred - can use defaults)

---

### 2.3 Scheduler API ⏭️

**Priority**: P0\
**Estimate**: 3 days\
**Dependencies**: 2.2, 2.4\
**Status**: Deferred (Requires Notes & Cards first)

**Note:** Scheduler requires proper note and card creation first.
The scheduler API will be implemented after we have:

- Notes API (2.4) - to create content
- Cards API (implied) - to have cards to review

Endpoints Planned:

- [ ] GET /api/v1/scheduler/next (get next card)
- [ ] POST /api/v1/scheduler/answer (answer card)
- [ ] GET /api/v1/scheduler/counts (review counts)
- [ ] GET /api/v1/scheduler/congrats (completion)

**Acceptance Criteria** (Future):

- Cards returned in correct order
- Answer updates scheduling
- Counts reflect queue state
- FSRS parameters applied

---

### 2.4 Notes API ✅

**Priority**: P0\
**Estimate**: 2 days\
**Dependencies**: 2.1\
**Status**: Core Complete

Endpoints Implemented:

- [x] GET /api/v1/notes/{id} - Get note by ID
- [x] POST /api/v1/notes - Create note
- [x] PUT /api/v1/notes/{id} - Update note
- [x] DELETE /api/v1/notes/{id} - Delete note
- [x] GET /api/v1/notes/{id}/cards - Get cards for note
- [ ] POST /api/v1/notes/batch - Batch create (future)
- [ ] PUT /api/v1/notes/batch - Batch update (future)
- [ ] POST /api/v1/notes/{id}/tags - Manage tags (future)

**Files Created**:

- `rslib/webapp/src/routes/notes.rs` - Note route handlers

**Acceptance Criteria** (Core Met):

- ✅ Notes create with correct fields
- ✅ Updates reflect in database
- ✅ Tags handled correctly
- ✅ Cards automatically generated from notes
- ⏭️ Batch operations (deferred - nice to have)

---

### 2.5 Cards API ✅

**Priority**: P0\
**Estimate**: 1 day\
**Dependencies**: 2.4\
**Status**: Complete

- [x] GET /api/v1/cards/{id}
- [x] PUT /api/v1/cards/{id}
- [x] DELETE /api/v1/cards/{id}
- [x] POST /api/v1/cards/{id}/flag
- [x] POST /api/v1/cards/{id}/suspend
- [x] POST /api/v1/cards/{id}/unsuspend
- [x] POST /api/v1/cards/{id}/bury
- [x] POST /api/v1/cards/batch (batch get)
- [x] POST /api/v1/cards/batch-update (batch update)

**Files Created**:

- `rslib/webapp/src/routes/cards.rs` - Card route handlers

**Files Enhanced**:

- `rslib/webapp/src/routes/mod.rs` - Added card route exports
- `rslib/webapp/src/server/router.rs` - Integrated card routes
- `rslib/webapp/src/openapi.rs` - Added Cards API documentation and schemas

**Acceptance Criteria** (All Met):

- ✅ Card CRUD operations work (get, update, delete)
- ✅ State changes (flag/suspend/bury/unsuspend) work
- ✅ Batch operations implemented (batch get and batch update)
- ✅ Routes integrated into router with authentication
- ✅ OpenAPI documentation complete
- ✅ All endpoints use proper service traits (CardsService, SchedulerService)

---

### 2.6 Search API ✅

**Priority**: P1\
**Estimate**: 1 day\
**Dependencies**: 2.1\
**Status**: Complete

- [x] POST /api/v1/search/cards
- [x] POST /api/v1/search/notes
- [x] POST /api/v1/search/find-replace

**Files Created**:

- `rslib/webapp/src/routes/search.rs` - Search route handlers

**Files Enhanced**:

- `rslib/webapp/src/routes/mod.rs` - Added search route exports
- `rslib/webapp/src/server/router.rs` - Integrated search routes
- `rslib/webapp/src/openapi.rs` - Added Search API documentation
- `rslib/webapp/Cargo.toml` - Added regex dependency

**Acceptance Criteria** (All Met):

- ✅ Search query syntax supported (Anki's native syntax)
- ✅ Results include all matching IDs
- ✅ Optional sorting by column with reverse option
- ✅ Find-replace works with regex and case-sensitive options
- ✅ Field-specific find-replace supported

---

### 2.7 Media API ✅ (Partial)

**Priority**: P1\
**Estimate**: 2 days\
**Dependencies**: 2.1\
**Status**: Partially Complete

- [x] POST /api/v1/media (upload)
- [x] DELETE /api/v1/media (trash files)
- [x] GET /api/v1/media/check (check for unused/missing)
- [ ] GET /api/v1/media/{filename} (not implemented - requires public media folder accessor)

**Files Created**:

- `rslib/webapp/src/routes/media.rs` - Media route handlers

**Files Enhanced**:

- `rslib/webapp/src/routes/mod.rs` - Added media route exports
- `rslib/webapp/src/server/router.rs` - Integrated media routes
- `rslib/webapp/Cargo.toml` - Added mime_guess dependency
- `rslib/webapp/src/error.rs` - Added not_implemented helper

**Acceptance Criteria**:

- ✅ Files upload successfully (multipart form-data)
- ⏭️ Correct MIME types served (pending get_media implementation)
- ✅ Media check finds unused/missing files
- ✅ Files can be moved to trash

**Note:** GET /api/v1/media/{filename} is stubbed but not fully implemented
due to private media_folder field in Collection. This would require either:

- Adding a public accessor to Collection
- Storing media path in session backend
- Using a different approach to file serving

---

### 2.8 Tags API ✅

**Priority**: P2\
**Estimate**: 1 day\
**Dependencies**: 2.1\
**Status**: Complete

- [x] GET /api/v1/tags - List all tags
- [x] GET /api/v1/tags/tree - Get tag tree structure
- [x] PUT /api/v1/tags/rename - Rename tags
- [x] DELETE /api/v1/tags/{name} - Delete tag
- [x] POST /api/v1/tags/clear-unused - Clear unused tags

**Files Created**:

- `rslib/webapp/src/routes/tags.rs` - Tags route handlers (210 lines)

**Files Enhanced**:

- `rslib/webapp/src/routes/mod.rs` - Added tags route exports
- `rslib/webapp/src/server/router.rs` - Integrated tags routes

**Acceptance Criteria** (All Met):

- ✅ Tags list correctly (all_tags service)
- ✅ Tag tree shows hierarchical structure
- ✅ Rename updates all notes using the tag
- ✅ Delete removes tag from all notes
- ✅ Clear unused removes tags not referenced by any notes

---

### 2.9 Statistics API ✅

**Priority**: P2\
**Estimate**: 1 day\
**Dependencies**: 2.1\
**Status**: Complete

- [x] GET /api/v1/stats/card/{id} - Get card-specific statistics
- [x] GET /api/v1/stats/collection - Get collection-wide statistics
- [x] GET /api/v1/stats/today - Get today's study statistics
- [ ] GET /api/v1/stats/graphs - Complex graphs data (stubbed - needs protobuf conversion)

**Files Created**:

- `rslib/webapp/src/routes/stats.rs` - Statistics route handlers (236 lines)

**Files Enhanced**:

- `rslib/webapp/src/routes/mod.rs` - Added stats route exports
- `rslib/webapp/src/server/router.rs` - Integrated stats routes

**Acceptance Criteria** (Mostly Met):

- ✅ Card stats provide comprehensive review history
- ✅ Collection stats show today's activity and card counts
- ✅ Today stats optimized for dashboard display
- ⏭️ Graphs endpoint stubbed (GraphsResponse lacks Serialize trait)

**Note:** The graphs endpoint is stubbed because GraphsResponse is a complex
nested protobuf structure that doesn't implement Serialize. To fully implement,
we would need custom protobuf-to-JSON conversion logic.

---

## Phase 3: UI Components (3 weeks)

### 3.1 Authentication UI ✅

**Priority**: P0\
**Estimate**: 2 days\
**Dependencies**: 1.3\
**Status**: Complete

- [x] Login page
- [x] Registration page
- [x] Profile page
- [x] Auth state management (stores)
- [x] Protected route wrapper
- [x] API client with auth headers

**Files Created**:

- `ts/routes/webapp/auth/login/+page.svelte` - Login page
- `ts/routes/webapp/auth/register/+page.svelte` - Registration page
- `ts/routes/webapp/auth/profile/+page.svelte` - Profile page
- `ts/routes/webapp/+layout.svelte` - Protected route wrapper
- `ts/routes/webapp/+page.svelte` - Dashboard home
- `ts/lib/webapp/stores/auth.ts` - Auth state management
- `ts/lib/webapp/api/client.ts` - API client with JWT support

**Acceptance Criteria** (All Met):

- ✅ User can login/register
- ✅ JWT stored securely in localStorage
- ✅ Auto-redirect on auth required
- ✅ Logout works
- ✅ Protected routes redirect to login
- ✅ API client includes auth headers

---

### 3.2 Collection Manager UI ✅

**Priority**: P0\
**Estimate**: 1 day\
**Dependencies**: 2.1, 3.1\
**Status**: Complete

- [x] Collection list page
- [x] Create collection dialog
- [x] Delete collection confirmation
- [x] Collection selection
- [x] Collection state management

**Files Created**:

- `ts/routes/webapp/collections/+page.svelte` - Collections management page
- `ts/lib/webapp/components/CollectionList.svelte` - Collection list component
- `ts/lib/webapp/components/CreateCollectionDialog.svelte` - Create dialog
- `ts/lib/webapp/stores/collection.ts` - Collection state store

**Files Enhanced**:

- `ts/lib/webapp/api/client.ts` - Added collection endpoints
- `ts/routes/webapp/+page.svelte` - Added collections link

**Acceptance Criteria** (All Met):

- ✅ User can create collections
- ✅ Can switch between collections
- ✅ Can delete collections with confirmation
- ✅ Collection state persisted in localStorage
- ✅ Current collection highlighted

---

### 3.3 Deck Browser UI ✅

**Priority**: P0\
**Estimate**: 3 days\
**Dependencies**: 2.2, 3.2\
**Status**: Complete

- [x] Deck tree display component
- [x] Study counts display
- [x] Quick study button
- [x] Create/rename/delete deck dialogs
- [x] Collection-aware deck management

**Files Created**:

- `ts/routes/webapp/decks/+page.svelte` - Deck browser page
- `ts/lib/webapp/components/DeckTree.svelte` - Deck tree component
- `ts/lib/webapp/components/DeckNode.svelte` - Individual deck display
- `ts/lib/webapp/components/DeckDialog.svelte` - Create/rename dialog

**Files Enhanced**:

- `ts/lib/webapp/api/client.ts` - Added deck endpoints
- `ts/routes/webapp/+page.svelte` - Added decks link

**Acceptance Criteria** (All Met):

- ✅ Deck tree displays correctly
- ✅ Counts display for new/learn/review
- ✅ Deck management works (create/rename/delete)
- ✅ Responsive design
- ✅ Collection-aware (requires active collection)

---

### 3.4 Reviewer UI 🚧

**Priority**: P0\
**Estimate**: 5 days\
**Dependencies**: 2.3, 3.2\
**Status**: In Progress - Backend API needs completion

- [x] Scheduler REST API routes created
- [ ] Card rendering integration (blocked)
- [ ] Card display component
- [ ] Question/answer reveal
- [ ] Answer buttons (Again/Hard/Good/Easy)
- [ ] Keyboard shortcuts
- [ ] Progress indicator
- [ ] Audio playback integration
- [ ] Card actions menu (edit/flag/suspend/bury)
- [ ] Undo/redo support
- [ ] Study completion screen integration

**Files Created (Partial)**:

- `rslib/webapp/src/routes/scheduler.rs` - Scheduler endpoints (needs rendering fix)
- Updated `rslib/webapp/src/routes/mod.rs` - Added scheduler module
- Updated `rslib/webapp/src/server/router.rs` - Added scheduler routes

**Files to Create**:

- `ts/routes/webapp/review/+page.svelte`
- `ts/lib/webapp/components/CardDisplay.svelte`
- `ts/lib/webapp/components/AnswerButtons.svelte`
- `ts/lib/webapp/components/ReviewProgress.svelte`
- `ts/lib/webapp/stores/reviewer.ts`

**Blockers**:

Card rendering requires deep integration with Anki's internal APIs (Note, Notetype, CardTemplate).
Need to either:

1. Create a simplified service wrapper for card rendering
2. Return raw card data and render client-side
3. Use a different rendering approach

**Acceptance Criteria**:

- Cards display correctly (HTML/CSS)
- Answer buttons update schedule
- Keyboard shortcuts work (1/2/3/4, space)
- Audio auto-plays if configured
- Undo works
- Progress accurate

---

### 3.5 Editor UI

**Priority**: P0\
**Estimate**: 5 days\
**Dependencies**: 2.4, 3.2

- [ ] Field editor components (rich text)
- [ ] Tag input with autocomplete
- [ ] Deck selector
- [ ] Notetype selector
- [ ] Card preview
- [ ] Media upload (drag-drop)
- [ ] Duplicate detection
- [ ] Add card button
- [ ] Form validation

**Files to Create**:

- `ts/routes/webapp/editor/+page.svelte`
- `ts/lib/webapp/components/FieldEditor.svelte`
- `ts/lib/webapp/components/TagInput.svelte`
- `ts/lib/webapp/components/DeckSelector.svelte`
- `ts/lib/webapp/components/CardPreview.svelte`
- `ts/lib/webapp/stores/editor.ts`

**Acceptance Criteria**:

- Fields editable with formatting
- Tags autocomplete
- Deck/notetype selectable
- Preview shows rendered card
- Media uploads work
- Duplicate warnings shown
- Cards save correctly

---

### 3.6 Card Browser UI

**Priority**: P1\
**Estimate**: 4 days\
**Dependencies**: 2.6, 3.2

- [ ] Search input with query builder
- [ ] Card/note list table
- [ ] Column selection/customization
- [ ] Multi-select rows
- [ ] Bulk operations toolbar
- [ ] Filter sidebar
- [ ] Preview pane
- [ ] Sorting
- [ ] Pagination

**Files to Create**:

- `ts/routes/webapp/browse/+page.svelte`
- `ts/lib/webapp/components/SearchBar.svelte`
- `ts/lib/webapp/components/CardTable.svelte`
- `ts/lib/webapp/components/FilterSidebar.svelte`
- `ts/lib/webapp/components/PreviewPane.svelte`

**Acceptance Criteria**:

- Search works correctly
- Table displays cards
- Multi-select functional
- Bulk operations work
- Preview shows card content
- Performant with large datasets

---

### 3.7 Statistics UI

**Priority**: P2\
**Estimate**: 2 days\
**Dependencies**: 2.9, 3.2

- [ ] Integrate existing graphs page
- [ ] Add deck-specific stats view
- [ ] Calendar heatmap
- [ ] Study time tracking
- [ ] Retention graphs

**Files to Create**:

- `ts/routes/webapp/stats/+page.svelte`
- `ts/lib/webapp/components/StatsOverview.svelte`

**Acceptance Criteria**:

- Graphs render correctly
- Data accurate
- Can switch between decks
- Responsive layout

---

### 3.8 Settings UI

**Priority**: P2\
**Estimate**: 2 days\
**Dependencies**: 3.1

- [ ] User preferences
- [ ] Collection settings
- [ ] Appearance settings (theme)
- [ ] Scheduling options
- [ ] Backup settings

**Files to Create**:

- `ts/routes/webapp/settings/+page.svelte`
- `ts/lib/webapp/components/SettingsPanel.svelte`

**Acceptance Criteria**:

- Settings persist
- Changes apply immediately or on save
- Validation works

---

### 3.9 Navigation & Layout

**Priority**: P0\
**Estimate**: 2 days\
**Dependencies**: 3.1

- [ ] Top navigation bar
- [ ] Sidebar menu
- [ ] User menu
- [ ] Breadcrumbs
- [ ] Mobile responsive layout
- [ ] Theme switching

**Files to Create**:

- `ts/routes/webapp/+layout.svelte`
- `ts/lib/webapp/components/NavBar.svelte`
- `ts/lib/webapp/components/Sidebar.svelte`
- `ts/lib/webapp/components/UserMenu.svelte`

**Acceptance Criteria**:

- Navigation works on all pages
- Mobile menu functional
- User can access profile/logout
- Theme persists

---

## Phase 4: Desktop Parity — Quick Wins

Tasks identified in `docs/webapp/FUNCTIONALITY_GAP_ANALYSIS.md` that close meaningful gaps against
the desktop app with relatively small effort. All required backend APIs already exist; most items
are frontend-only or require only minor Rust changes.

### 4.1 Critical Bug Fixes ✅

**Priority**: P0\
**Estimate**: 1 day\
**Dependencies**: Phase 3 complete\
**Source**: FUNCTIONALITY_GAP_ANALYSIS.md\
**Status**: Complete

- [x] **Fix deck-scoped study** — `scheduler.rs` ignores `deck_id`; `get_queued_cards` is called
      without a deck filter so studying always draws from the whole collection. Pass the deck ID
      through to the scheduler call.
- [x] **Fix `PUT /api/v1/decks` still listed as future** in TASKS.md — endpoint is already
      implemented (router.rs) but task 2.2 still marks it as unchecked.

**Files Modified**:

- `rslib/webapp/src/routes/scheduler.rs` - Added `set_current_deck()` call before getting queued cards
- `TASKS.md` - Marked PUT /api/v1/decks as implemented

**Acceptance Criteria** (All Met):

- ✅ Studying a deck only presents cards from that deck (and its children)
- ✅ TASKS.md accurately reflects implemented state

---

### 4.2 Interval Preview on Answer Buttons ✅

**Priority**: P0\
**Estimate**: 1 day\
**Dependencies**: Phase 2.3 (Scheduler API ✅)\
**Effort**: Low — call `DescribeNextStates` RPC, render text above buttons\
**Source**: Gap analysis §3 — "Next interval preview"\
**Status**: Complete

- [x] Add `GET /api/v1/scheduler/decks/{deck_id}/cards/{card_id}/next-states` endpoint that calls the
      `DescribeNextStates` RPC
- [x] Update `AnswerButtons.svelte` to fetch and display the interval string above each button
      (e.g. `<1m`, `10m`, `1d`, `4d`)
- [x] Show a loading skeleton until intervals arrive; fall back gracefully if the call fails

**Files Created**:

- `rslib/webapp/src/routes/scheduler.rs` - Added `get_next_states` function

**Files Modified**:

- `rslib/webapp/src/routes/mod.rs` - Exported `get_next_states`
- `rslib/webapp/src/server/router.rs` - Added route and import
- `rslib/webapp/src/openapi.rs` - Added API documentation and `NextStatesResponse` schema

**Files Modified (Frontend)**:

- `ts/lib/webapp/api/client.ts` - Added `getNextStates()` method
- `ts/lib/webapp/components/AnswerButtons.svelte` - Added `intervals` prop with loading skeleton / graceful fallback
- `ts/routes/webapp/review/+page.svelte` - Added `fetchIntervals()` called after each card load

**Acceptance Criteria** (All Met):

- ✅ Backend endpoint returns interval descriptions (again, hard, good, easy)
- ✅ Answer buttons show human-readable next-review interval (`<1m`, `10m`, `1d`, `4d`)
- ✅ Intervals update after each answer
- ✅ Loading skeleton shown while fetching; buttons still work if fetch fails

---

### 4.3 Time Tracking Per Card ✅

**Priority**: P1\
**Estimate**: 2 hours\
**Dependencies**: Reviewer UI (3.4 ✅)\
**Effort**: Trivial — frontend only\
**Source**: Gap analysis §3 — "Time tracking"\
**Status**: Complete

- [x] Record `Date.now()` when a card is displayed and when an answer is submitted
- [x] Pass `milliseconds_taken` in the answer request body to `POST .../answer`
- [x] Update `AnswerCardRequest` schema in `openapi.rs`
- [x] Update the Rust handler to forward `milliseconds_taken` to `CardAnswer`

**Files Modified**:

Backend:

- `rslib/webapp/src/routes/scheduler.rs` - Added `milliseconds_taken` field to `AnswerCardRequest`, passed to `CardAnswer`
- `rslib/webapp/src/openapi.rs` - Fixed `AnswerCardRequest` schema (changed `ease` to `rating` and added `milliseconds_taken`)

Frontend:

- `ts/lib/webapp/api/client.ts` - Updated `answerCard()` to accept and send `milliseconds_taken`
- `ts/routes/webapp/review/+page.svelte` - Added `cardStartTime` tracking, calculates time spent on each card

**Acceptance Criteria** (All Met):

- ✅ `AnswerCardRequest` accepts optional `milliseconds_taken` field (defaults to 0)
- ✅ Backend forwards `milliseconds_taken` to `CardAnswer` (which stores it in the revlog)
- ✅ Frontend tracks time from card display to answer submission
- ✅ Time spent is visible in card stats (tracked by Anki core)
- ✅ OpenAPI documentation updated and corrected

**Implementation Details**:

- Timer starts when card loads (`Date.now()`)
- Timer stops when user clicks answer button
- Elapsed time sent as `milliseconds_taken` in POST request
- Anki stores in `revlog.time` column for statistics
- Visible via `/api/v1/stats/card/{id}` (`average_secs`, `total_secs`) and `/api/v1/stats/today` (`answer_millis`)

---

### 4.4 Flag / Suspend / Bury During Review ✅

**Priority**: P1\
**Estimate**: 1 day\
**Dependencies**: Cards API (2.5 ✅), Reviewer UI (3.4 ✅)\
**Effort**: Low — APIs exist, only UI work required\
**Source**: Gap analysis §3 — "Flag cards", "Suspend/bury during review"\
**Status**: Complete

- [x] Add a context menu or "More" button in `review/+page.svelte`
- [x] Wire "Flag" (colours 1–4), "Suspend", and "Bury" actions to existing API calls
- [x] After flag/suspend/bury, automatically load the next card

**Files Created**:

- `ts/lib/webapp/components/CardActions.svelte` - "More" dropdown with Flag submenu (colors 1–4), Suspend, Bury

**Files Modified**:

- `ts/routes/webapp/review/+page.svelte` - Integrated `CardActions` in header; `on:action` triggers `loadNextCard()`
- `ts/lib/webapp/stores/reviewer.ts` - Added `flags?: number` to `Card` interface
- `rslib/webapp/src/routes/scheduler.rs` - Added `flags: u8` to `QueuedCardResponse`

**Acceptance Criteria** (All Met):

- ✅ User can flag (colors 1–4 + clear), suspend, or bury the current card without leaving the reviewer
- ✅ Current flag color shown as a dot on the "More" button
- ✅ After any action, next card is loaded automatically
- ✅ Click-outside handler closes the dropdown cleanly

---

### 4.5 Cloze Deletion Toolbar Helper ✅

**Priority**: P1\
**Estimate**: 2 hours\
**Dependencies**: Editor UI (3.5 ✅)\
**Effort**: Trivial — frontend only\
**Source**: Gap analysis §2 — "Cloze deletion helper"\
**Status**: Complete

- [x] Add a `[c1]` toolbar button above cloze-type field editors in `FieldEditor.svelte`
- [x] Button wraps the current text selection in `{{c1::…}}` and auto-increments the cloze index
- [x] Add keyboard shortcut `Ctrl+Shift+C` (mirrors desktop)

**Acceptance Criteria** (All Met):

- ✅ Cloze syntax inserted correctly; index increments automatically per card

---

### 4.6 Sticky Fields in Editor ✅

**Priority**: P2\
**Estimate**: 2 hours\
**Dependencies**: Editor UI (3.5 ✅)\
**Effort**: Trivial — frontend only\
**Source**: Gap analysis §2 — "Sticky fields"\
**Status**: Complete

- [x] Add a pin icon per field in `FieldEditor.svelte`
- [x] Pinned fields retain their value after a successful card submission
- [x] Persist sticky-field flags in `localStorage` per notetype

**Acceptance Criteria** (All Met):

- ✅ Pinned fields keep their content between card additions

---

### 4.7 Duplicate Detection in Editor ✅

**Priority**: P1\
**Estimate**: 1 day\
**Dependencies**: Notes API (2.4 ✅), Editor UI (3.5 ✅)\
**Effort**: Low — `NoteFieldsCheck` RPC exists\
**Source**: Gap analysis §2 — "Duplicate detection"\
**Status**: Complete

- [x] Add `POST /api/v1/notes/check-fields` endpoint wrapping `NoteFieldsCheck`
- [x] Call the endpoint on change of the first field in the editor (debounced 500 ms)
- [x] Show a non-blocking warning banner if a duplicate is found (`state === 2`)

**Acceptance Criteria** (All Met):

- ✅ Warning appears when the first field matches an existing note's first field

---

### 4.8 Deck Collapse / Expand State ✅

**Priority**: P2\
**Estimate**: 2 hours\
**Dependencies**: Decks API (2.2 ✅), Deck Browser UI (3.3 ✅)\
**Effort**: Trivial — `PUT /api/v1/decks/{id}` already accepts `collapsed`\
**Source**: Gap analysis §1 — "Collapse/expand state"\
**Status**: Complete

- [x] Persist collapse state via `PUT /api/v1/decks/{id}` when user toggles a deck node
- [x] Read initial state from the deck tree response on page load

**Acceptance Criteria** (All Met):

- ✅ Collapse state survives page refresh

---

### 4.9 Overview Screen Before Study ✅

**Priority**: P2\
**Estimate**: 1 day\
**Dependencies**: Deck Browser UI (3.3 ✅), Reviewer UI (3.4 ✅)\
**Effort**: Low — data already available\
**Source**: Gap analysis §3 — "Overview screen"\
**Status**: Complete

- [x] Add `review/overview/+page.svelte` shown between clicking "Study" and entering the reviewer
- [x] Display deck name, description, and new/learn/review counts
- [x] Provide a "Start Study" button to enter the reviewer and a "Back" link

**Acceptance Criteria** (All Met):

- ✅ Overview displays correct card counts; "Start Study" enters the reviewer

---

### 4.10 Audio Playback During Review ✅

**Priority**: P2\
**Estimate**: 1 day\
**Dependencies**: Reviewer UI (3.4 ✅), Media API (2.7 ✅)\
**Effort**: Low — `[sound:file.mp3]` tags already in rendered HTML\
**Source**: Gap analysis §3 — "Audio replay"\
**Status**: Complete

- [x] Parse `[sound:…]` tags in the rendered card HTML inside `CardDisplay.svelte`
- [x] Replace each tag with an HTML `<audio>` element pointing to `/api/v1/media/{filename}?token=…`
- [x] Auto-play the first audio file when the card appears; replay button for subsequent files

**Note**: Auth middleware extended to accept `?token=` query param so `<audio src>` browser
requests (which cannot send Authorization headers) are authenticated correctly.

**Acceptance Criteria** (All Met):

- ✅ Audio files embedded in cards play automatically and can be replayed

---

### 4.11 Keyboard Shortcuts in Browse and Editor ✅

**Priority**: P2\
**Estimate**: 4 hours\
**Dependencies**: Card Browser UI (3.6 ✅), Editor UI (3.5 ✅)\
**Effort**: Trivial — frontend only\
**Source**: Gap analysis — "Keyboard navigation"\
**Status**: Complete

- [x] `Ctrl+Enter` to submit card in Editor
- [x] `Ctrl+F` to focus search bar in Browser
- [x] `Delete` to delete selected cards in Browser
- [x] `Escape` to clear selection in Browser

**Acceptance Criteria** (All Met):

- ✅ All listed shortcuts work without interfering with browser defaults

---

### 4.12 APKG Import ✅

**Priority**: P1\
**Estimate**: 2 days\
**Dependencies**: Media API (2.7 ✅), Navigation (3.9 ✅)\
**Effort**: Medium — Rust core supports it, needs API plumbing and UI\
**Source**: Gap analysis — cross-cutting "Import/Export"
**Status**: Complete

- [x] Add `POST /api/v1/import/apkg` endpoint wrapping `import_anki_package`
- [x] Save uploaded multipart file to temporary storage for processing
- [x] Create `import/+page.svelte` with file drop zone and processing state
- [x] Display results summary (notes added/updated, media imported)

**Acceptance Criteria**:

- ✅ User can upload an `.apkg` file and have its contents merged into their collection
- ✅ Media files from the package are correctly placed in the user's media folder
- ✅ UI shows clear progress and success/error reporting

---

### 4.13 APKG Export

**Priority**: P1\
**Estimate**: 1 day\
**Dependencies**: Navigation (3.9 ✅), Decks API (2.2 ✅)\
**Effort**: Low — Rust core handles the packaging, needs endpoint and download trigger\
**Source**: Gap analysis — cross-cutting "Import/Export"

- [ ] Add `POST /api/v1/export/apkg` endpoint wrapping `export_anki_package`
- [ ] Implement deck-specific and collection-wide export logic
- [ ] Add "Export" action to Deck menu and Browser toolbar
- [ ] Implement file download trigger in frontend

**Acceptance Criteria**:

- User can export a deck or the entire collection as a valid `.apkg` file
- Export options (like including scheduling) are respected
- Download starts automatically and contains the correct data/media

---

## Phase 5: Polish & Testing

### 5.1 API Testing ✅

**Priority**: P0\
**Estimate**: 3 days\
**Dependencies**: Phase 4 complete

- [x] Write integration tests for auth
- [x] Write integration tests for collections
- [x] Write integration tests for scheduler
- [x] Write integration tests for CRUD operations
- [ ] Write load tests
- [x] Test error handling
- [x] Test edge cases

**Files Created**:

- `rslib/webapp/tests/auth_test.rs`
- `rslib/webapp/tests/collection_test.rs`
- `rslib/webapp/tests/deck_test.rs`
- `rslib/webapp/tests/note_test.rs`
- `rslib/webapp/tests/card_test.rs`
- `rslib/webapp/tests/scheduler_test.rs`
- `rslib/webapp/tests/search_test.rs`
- `rslib/webapp/tests/stats_test.rs`
- `rslib/webapp/tests/media_test.rs`
- `test-webapp.sh`

**Acceptance Criteria**:

- 80%+ code coverage
- All critical paths tested
- Load tests pass (deferred)
- Edge cases handled

---

### 5.2 UI Testing

**Priority**: P1\
**Estimate**: 2 days\
**Dependencies**: Phase 4 complete

- [ ] Write component tests
- [ ] Write E2E tests (Playwright)
- [ ] Test keyboard shortcuts
- [ ] Test responsive layouts
- [ ] Cross-browser testing

**Files to Create**:

- `ts/tests/webapp/reviewer.test.ts`
- `ts/tests/webapp/editor.test.ts`
- `ts/tests/e2e/study-flow.spec.ts`

**Acceptance Criteria**:

- Critical user flows tested
- Works in Chrome/Firefox/Safari
- Mobile layout tested

---

### 5.3 Documentation

**Priority**: P1\
**Estimate**: 2 days\
**Dependencies**: All features complete

- [ ] API documentation (OpenAPI/Swagger)
- [ ] User guide
- [ ] Admin guide
- [ ] Deployment guide
- [ ] Contributing guide
- [ ] Update README.webapp.md

**Files to Create**:

- `docs/webapp/api.yaml`
- `docs/webapp/user-guide.md`
- `docs/webapp/admin-guide.md`
- `docs/webapp/deployment.md`

**Acceptance Criteria**:

- API fully documented
- User can self-serve setup
- Common issues documented

---

### 5.4 Security Audit

**Priority**: P0\
**Estimate**: 2 days\
**Dependencies**: All features complete

- [ ] Review authentication implementation
- [ ] Test authorization boundaries
- [ ] Check for SQL injection vulnerabilities
- [ ] Check for XSS vulnerabilities
- [ ] Review file upload security
- [ ] Test rate limiting
- [ ] Review session management
- [ ] Dependency security scan

**Acceptance Criteria**:

- No critical vulnerabilities
- Auth boundaries enforced
- Input validation complete
- Rate limiting works

---

### 5.5 Performance Optimization

**Priority**: P1\
**Estimate**: 2 days\
**Dependencies**: Testing complete

- [ ] Profile API endpoints
- [ ] Optimize database queries
- [ ] Add response caching
- [ ] Optimize frontend bundle size
- [ ] Add lazy loading
- [ ] Database indexing
- [ ] Connection pooling tuning

**Acceptance Criteria**:

- API responds < 100ms for common ops
- Page load < 2s
- Memory usage stable
- Can handle 100 concurrent users

---

### 5.6 Deployment Packaging

**Priority**: P1\
**Estimate**: 3 days\
**Dependencies**: All features complete

- [ ] Create binary release scripts
- [ ] Create Docker image
- [ ] Create docker-compose setup
- [ ] Create systemd service file
- [ ] Add health check endpoint
- [ ] Add version endpoint
- [ ] Create install script
- [ ] Test deployment on various platforms

**Files to Create**:

- `Dockerfile`
- `docker-compose.yml`
- `deploy/anki-webapp.service`
- `deploy/install.sh`

**Acceptance Criteria**:

- Docker image builds
- docker-compose works out of box
- Service auto-starts on boot
- Health checks work

---

## Optional Enhancements (Future)

### Rich Text Editor

**Priority**: P1\
**Estimate**: 5 days\
**Source**: Gap analysis §2 — largest functional gap

- [ ] Replace `<textarea>` fields with a rich text editor (ProseMirror or TipTap)
- [ ] Toolbar: bold, italic, underline, ordered/unordered list, text colour
- [ ] Toggle between rich-text and raw HTML per field (mirrors desktop)
- [ ] Paste images from clipboard directly into fields; upload to media API

---

### Media Attachments in Note Fields

**Priority**: P1\
**Estimate**: 2 days\
**Source**: Gap analysis §2 — "Media attachments"

- [ ] Drag-and-drop file onto a field editor to upload and embed `<img>` / `[sound:…]` tag
- [ ] Attachment button in field toolbar to open file picker
- [ ] Files uploaded via `POST /api/v1/media`; tag inserted at cursor position

---

### Deck Options / Configuration UI

**Priority**: P1\
**Estimate**: 4 days\
**Source**: Gap analysis §1 — "Deck options/config"

- [ ] Backend: expose `GET/PUT /api/v1/decks/{id}/config` wrapping the existing protobuf deck
      config RPCs
- [ ] Frontend: settings modal for new-card limit, review limit, learning steps, lapse settings,
      and FSRS parameters
- [ ] Changes persist to the deck's config group

---

### Import / Export UI

**Priority**: P1\
**Estimate**: 3 days\
**Source**: Gap analysis — cross-cutting "Import/Export"

- [ ] Import `.apkg` wizard — upload file, call backend importer, show progress
- [ ] Import CSV wizard
- [ ] Export deck as `.apkg` — backend endpoint + download trigger
- [ ] Export full collection as `.colpkg`

---

### Filtered Decks / Custom Study

**Priority**: P2\
**Estimate**: 4 days\
**Source**: Gap analysis §1 & §3 — "Filtered/custom study decks"

- [ ] Backend: `POST /api/v1/decks/filtered` wrapping `GetOrCreateFilteredDeck` +
      `RebuildFilteredDeck`
- [ ] Custom study dialog (extend daily limits, review ahead, study by tag, preview new)
- [ ] Filtered deck shown in deck tree with rebuild/empty actions

---

### Note Type Management

**Priority**: P2\
**Estimate**: 5 days\
**Source**: Gap analysis §2 — "Note type management"

- [ ] List, create, rename, and delete note types
- [ ] Add / remove / reorder fields
- [ ] Edit card template HTML (front / back) and CSS
- [ ] Change sort field
- [ ] Change note type for existing notes with field mapping dialog

---

### Real-time Sync

**Priority**: P3\
**Estimate**: 1 week

- [ ] WebSocket implementation
- [ ] Real-time progress updates
- [ ] Live collaboration (multiple users)

---

### Mobile Optimization

**Priority**: P2\
**Estimate**: 3 days

- [ ] Touch gesture support (swipe to reveal / rate)
- [ ] Mobile-specific layouts
- [ ] PWA manifest + install prompt
- [ ] Offline support with service worker

---

### Advanced Features

**Priority**: P3\
**Estimate**: 2+ weeks

- [ ] Collaborative decks
- [ ] Social features
- [ ] Advanced analytics
- [ ] AI suggestions
- [ ] Plugin system

---

## Timeline Summary

| Phase                              | Duration      | Dependencies | Status     |
|------------------------------------|---------------|--------------|------------|
| Phase 1: Foundation                | 2 weeks       | None         | ✅ Done     |
| Phase 2: Core API                  | 2 weeks       | Phase 1      | ✅ Done     |
| Phase 3: UI Components             | 3 weeks       | Phase 1, 2   | ✅ Done     |
| Phase 4: Desktop Parity Quick Wins | 1–2 weeks     | Phase 3      | 📋 Next    |
| Phase 5: Polish & Testing          | 2 weeks       | Phase 4      | 📋 Planned |
| Optional Enhancements              | ongoing       | Phase 5      | 💡 Future  |
| **Total (through Phase 5)**        | **~11 weeks** |              |            |

## Resource Requirements

- **Developer(s)**: 1-2 full-stack developers
- **Skills Required**: Rust, TypeScript, Svelte, SQL, REST APIs
- **Infrastructure**: Development server, test environment

## Risk Assessment

| Risk                     | Likelihood | Impact   | Mitigation                       |
|--------------------------|------------|----------|----------------------------------|
| Scope creep              | High       | High     | Stick to MVP, defer enhancements |
| Security vulnerabilities | Medium     | High     | Security audit, code review      |
| Performance issues       | Medium     | Medium   | Load testing, profiling          |
| Browser compatibility    | Low        | Medium   | Cross-browser testing            |
| Data corruption          | Low        | Critical | Extensive testing, backups       |

## Success Criteria

- [ ] Users can study cards via web browser
- [ ] API fully functional for all core operations
- [ ] Performance comparable to desktop app
- [ ] Security audit passes
- [ ] Documentation complete
- [ ] Can deploy on standard VPS
- [ ] Existing collections compatible
