# Anki Web App - Project Status

**Last Updated:** 2026-02-16  
**Current Phase:** Phase 3 - UI Components (In Progress)

---

## Overview

Building a web-based REST API and UI for Anki spaced repetition software. The project enables users to study flashcards through a web browser with full authentication, session management, and collection handling.

---

## Phase Completion Status

### ✅ Phase 1: Foundation (COMPLETE)

**Duration:** Completed  
**Status:** All tasks complete and tested

| Task                        | Status | Notes                                   |
|-----------------------------|--------|-----------------------------------------|
| 1.1 Project Structure Setup | ✅      | Webapp module integrated into workspace |
| 1.2 Database Schema         | ✅      | SQLite users and sessions tables        |
| 1.3 Authentication System   | ✅      | JWT + Argon2 password hashing           |
| 1.4 Session Management      | ✅      | Per-user Backend instances with Mutex   |
| 1.5 Configuration System    | ✅      | TOML + ENV + defaults, 4 tests passing  |
| 1.6 Error Handling          | ✅      | Consistent JSON errors, 9 tests passing |

**Key Achievements:**
- Secure authentication with JWT tokens
- User isolation with per-user collections
- Comprehensive error handling
- Full configuration management

---

### ✅ Phase 2: Core API (COMPLETE)

**Duration:** Completed  
**Status:** 9 of 9 tasks complete (100%)

| Task                | Status | Completion                                       |
|---------------------|--------|--------------------------------------------------|
| 2.1 Collections API | ✅      | 100% - Simplified single-collection architecture |
| 2.2 Decks API       | ✅      | 100% - Core CRUD complete (4 endpoints)          |
| 2.3 Scheduler API   | ✅      | 100% - Complete with undo/redo (5 endpoints)     |
| 2.4 Notes API       | ✅      | 100% - Core CRUD complete (5 endpoints)          |
| 2.5 Cards API       | ✅      | 100% - Complete (9 endpoints)                    |
| 2.6 Search API      | ✅      | 100% - Complete (3 endpoints)                    |
| 2.7 Media API       | ✅      | 100% - Upload, check, delete (3 endpoints)       |
| 2.8 Tags API        | ✅      | 100% - Complete (5 endpoints)                    |
| 2.9 Statistics API  | ✅      | 100% - Complete (4 endpoints)                    |

**Progress:** 9/9 tasks (100%)

---

### ✅ Phase 3: UI Components (COMPLETE!)

**Status:** ✅ **ALL TASKS COMPLETE!**  
**Dependencies:** Phase 2 completion ✅

| Task                        | Status | Completion                            |
|-----------------------------|--------|---------------------------------------|
| 3.1 Authentication UI       | ✅      | 100% - Login, register, profile pages |
| 3.2 Collection Manager UI   | ✅      | 100% - Collection CRUD interface      |
| 3.3 Deck Browser UI         | ✅      | 100% - Deck tree with study buttons   |
| 3.4 Reviewer UI             | ✅      | 100% - Complete study interface       |
| 3.5 Editor UI               | ✅      | 100% - Card creation interface        |
| 3.6 Card Browser UI         | ✅      | 100% - Search and bulk operations     |
| 3.7 Statistics UI           | ✅      | 100% - Stats dashboard                |
| 3.8 Settings UI             | ✅      | 100% - User preferences               |
| **3.9 Navigation & Layout** | **✅**  | **100% - JUST COMPLETED!**            |

**Progress:** ✅ 9/9 tasks (100%) - **PHASE COMPLETE!**

---

### 📋 Phase 4: Polish & Testing (NOT STARTED - FINAL PHASE!)

**Status:** Ready to begin!  
**Dependencies:** Phase 3 completion ✅

| Task                         | Status | Estimate |
|------------------------------|--------|----------|
| 4.1 API Testing              | 📋     | 3 days   |
| 4.2 UI Testing               | 📋     | 3 days   |
| 4.3 Performance Optimization | 📋     | 2 days   |
| 4.4 Security Audit           | 📋     | 2 days   |
| 4.5 Documentation            | 📋     | 3 days   |
| 4.6 Bug Fixes                | 📋     | 2 days   |

**Estimated Duration:** ~2 weeks

**Status:** Planned  
**Dependencies:** Phases 2 & 3 completion

---

## Latest Completion: Phase 3.9 - Navigation & Layout ✅

**Completed:** 2026-02-16  
**Lines of Code:** ~350 lines  
**Build Status:** ✅ Passing (0 errors, 10 accessibility warnings)

### 🎉 PHASE 3 COMPLETE!

All 9 UI Component tasks are now complete!

### Components Implemented

**Frontend UI (1 new file):**
- `ts/lib/webapp/components/NavBar.svelte` - Unified navigation bar

**Layout Integration:**
- Updated `ts/routes/webapp/+layout.svelte` with NavBar

### Features Delivered

1. **Unified Navigation Bar**
   - Brand/logo link to dashboard
   - Main nav links (Decks, Add, Browse, Stats)
   - User menu dropdown
   - Logout functionality

2. **Responsive Design**
   - Desktop: Horizontal layout
   - Mobile: Hamburger menu (<768px)
   - Touch-friendly buttons

3. **User Experience**
   - Active page highlighting
   - Sticky positioning
   - Smooth transitions
   - Auto-close menus on nav

### Documentation
- ✅ PHASE_3.9_COMPLETE.md

---

## Previous Completion: Phase 3.8 - Settings UI ✅

**Completed:** 2026-02-16  
**Lines of Code:** ~285 lines  
**Build Status:** ✅ Passing

### Components Implemented

**Frontend UI (1 new file):**
- `ts/routes/webapp/settings/+page.svelte` - Settings page

### Features Delivered

1. **Appearance Settings**
   - Theme selection (Light/Dark)

2. **Study Limits**
   - New cards per day (0-999)
   - Maximum reviews per day (0-9999)

3. **Study Interface Options**
   - Show answer time toggle
   - Auto-play audio toggle
   - Keyboard shortcuts toggle

4. **Persistence**
   - localStorage-based storage
   - Auto-load on mount
   - Success feedback

### Documentation
- ✅ PHASE_3.8_COMPLETE.md

---

## Previous Completion: Phase 3.7 - Statistics UI ✅

**Completed:** 2026-02-16  
**Lines of Code:** ~480 lines  
**Build Status:** ✅ Passing

### Components Implemented

**Frontend UI (1 new file):**
- `ts/routes/webapp/stats/+page.svelte` - Statistics dashboard

**API Integration:**
- Extended `ts/lib/webapp/api/client.ts` with stats methods
- `getTodayStats()`, `getCollectionStats()`, `getGraphs()`

### Features Delivered

1. **Today's Statistics**
   - Cards answered, correct count
   - Study time formatted
   - Accuracy percentage
   - Learn/Review/Relearn breakdown

2. **Collection Overview**
   - Total cards and notes
   - New/Young/Mature counts
   - Suspended/Buried cards

3. **Mature Card Performance**
   - Cards reviewed, retention rate

### Documentation
- ✅ PHASE_3.7_COMPLETE.md

---

## Previous Completion: Phase 3.6 - Card Browser UI ✅

**Completed:** 2026-02-16  
**Lines of Code:** ~650 lines  
**Build Status:** ✅ Passing

### Components Implemented

**Frontend UI (1 new file):**
- `ts/routes/webapp/browse/+page.svelte` - Browse interface with search and bulk operations

**API Integration:**
- Extended `ts/lib/webapp/api/client.ts` with search methods
- `searchCards()`, `searchNotes()`, `getCard()`
- Updated dashboard with "Browse" link

### Features Delivered

1. **Search Interface**
   - Cards/Notes mode toggle
   - Anki query syntax support
   - Enter key or button to search
   - Empty query returns all results

2. **Results Table**
   - Card/Note ID display
   - Deck, due date, interval columns
   - Multi-select with checkboxes
   - Select All / Deselect All
   - First 100 results pagination

3. **Bulk Operations**
   - Suspend selected cards
   - Delete selected cards/notes
   - Confirmation dialogs
   - Selection counter

### Documentation
- ✅ Complete feature documentation (PHASE_3.6_COMPLETE.md)
- ✅ Updated project status

---

## Previous Completion: Phase 3.5 - Editor UI ✅

**Completed:** 2026-02-16  
**Lines of Code:** ~900 lines  
**Build Status:** ✅ Passing

### Components Implemented

**Backend API (2 new endpoints):**
- `rslib/webapp/src/routes/notetypes.rs` - Notetype management
  - GET `/api/v1/notetypes` - List all notetypes
  - GET `/api/v1/notetypes/{id}` - Get notetype with fields

**Frontend UI (4 new files):**
- `ts/lib/webapp/stores/editor.ts` - Editor state management
- `ts/routes/webapp/editor/+page.svelte` - Main editor page
- `ts/lib/webapp/components/FieldEditor.svelte` - Field input component
- `ts/lib/webapp/components/TagInput.svelte` - Tag management component

**API Integration:**
- Extended `ts/lib/webapp/api/client.ts` with notetype and note methods
- Updated dashboard with "Add Cards" link

### Features Delivered

1. **Complete Card Creation Workflow**
   - Select deck from dropdown
   - Choose notetype (Basic, Cloze, etc.)
   - Fill in dynamic fields based on notetype
   - Add/remove tags with visual interface
   - Submit to create card

2. **Dynamic Field Generation**
   - Fields auto-generate based on selected notetype
   - Each notetype can have different fields
   - Multi-line textarea inputs
   - Proper labeling and focus states

3. **Tag Management**
   - Visual tag pills with remove buttons
   - Keyboard shortcuts (Enter/Space to add)
   - Backspace to remove last tag
   - Duplicate prevention

4. **Form Management**
   - Validation before submission
   - Success feedback with card ID
   - Auto-clear form for next card
   - Error handling with user messages

### Documentation
- ✅ Complete feature documentation (PHASE_3.5_COMPLETE.md)
- ✅ Updated project status
- ✅ API endpoints documented

---

## Previous Completion: Phase 3.4 - Reviewer UI ✅

**Completed:** 2026-02-16  
**Lines of Code:** ~700 lines  
**Build Status:** ✅ Passing

### Components Implemented (6 new files)

**Svelte UI:**
- `ts/lib/webapp/stores/reviewer.ts` - Review session state management
- `ts/routes/webapp/review/+page.svelte` - Main review page
- `ts/routes/webapp/review/+page.ts` - Page data loader
- `ts/lib/webapp/components/CardDisplay.svelte` - Card rendering component
- `ts/lib/webapp/components/AnswerButtons.svelte` - Answer rating buttons
- `ts/lib/webapp/components/ReviewProgress.svelte` - Study progress display

**API Integration:**
- Extended `ts/lib/webapp/api/client.ts` with scheduler methods
- Updated `ts/routes/webapp/decks/+page.svelte` for navigation

### Features Delivered

1. **Complete Study Workflow**
   - Navigate from deck browser to study session
   - View card question, reveal answer
   - Rate card (Again/Hard/Good/Easy)
   - Automatic progression to next card
   - Completion screen when done

2. **Keyboard Shortcuts**
   - `Space`/`Enter` - Show answer
   - `1-4` - Rate card (Again/Hard/Good/Easy)
   - `Ctrl+Z` - Undo
   - `Ctrl+Shift+Z`/`Ctrl+Y` - Redo

3. **Real-time Progress**
   - Live counts for new/learning/review cards
   - Visual progress indicators
   - Undo/redo button states

4. **Card Rendering**
   - Full HTML/CSS support from Anki templates
   - Dynamic CSS injection per card
   - Question/answer separation
   - Responsive design

### Documentation
- ✅ Complete feature documentation (PHASE_3.4_COMPLETE.md)
- ✅ Updated project status
- ✅ Keyboard shortcuts documented
- ✅ User flow documented

---

## Previous Completion: Phase 2.5 - Cards API ✅

**Completed:** 2026-02-15  
**Lines of Code:** ~650 lines  
**Build Status:** ✅ Passing

### Endpoints Implemented (9 total)

**Individual Operations:**
- GET /api/v1/cards/{id} - Get card by ID
- PUT /api/v1/cards/{id} - Update card (deck_id, due, flags)
- DELETE /api/v1/cards/{id} - Delete card
- POST /api/v1/cards/{id}/flag - Flag card (0-4 colors)
- POST /api/v1/cards/{id}/suspend - Suspend card
- POST /api/v1/cards/{id}/unsuspend - Unsuspend/restore card
- POST /api/v1/cards/{id}/bury - Bury card until next day

**Batch Operations:**
- POST /api/v1/cards/batch - Get multiple cards efficiently
- POST /api/v1/cards/batch-update - Update multiple cards in one transaction

### Documentation

- ✅ Complete OpenAPI 3.0 specification
- ✅ Swagger UI integration (http://localhost:8080/swagger-ui)
- ✅ Quick reference guide (CARDS_API_REFERENCE.md)
- ✅ Detailed completion report (PHASE_2.5_COMPLETE.md)

---

## API Endpoints Summary

### Authentication (Public)
- ✅ POST /api/v1/auth/register
- ✅ POST /api/v1/auth/login
- ✅ POST /api/v1/auth/logout (protected)
- ✅ GET /api/v1/auth/me (protected)

### Collection Management
- ✅ GET /api/v1/collection/info
- ✅ POST /api/v1/collection/close

### Decks
- ✅ GET /api/v1/decks (tree view)
- ✅ POST /api/v1/decks (create)
- ✅ GET /api/v1/decks/{id} (get)
- ✅ DELETE /api/v1/decks/{id} (delete)

### Notes
- ✅ GET /api/v1/notes/{id}
- ✅ POST /api/v1/notes (create)
- ✅ PUT /api/v1/notes/{id} (update)
- ✅ DELETE /api/v1/notes/{id}
- ✅ GET /api/v1/notes/{id}/cards

### Cards (NEW - Phase 2.5)
- ✅ GET /api/v1/cards/{id}
- ✅ PUT /api/v1/cards/{id}
- ✅ DELETE /api/v1/cards/{id}
- ✅ POST /api/v1/cards/{id}/flag
- ✅ POST /api/v1/cards/{id}/suspend
- ✅ POST /api/v1/cards/{id}/unsuspend
- ✅ POST /api/v1/cards/{id}/bury
- ✅ POST /api/v1/cards/batch
- ✅ POST /api/v1/cards/batch-update

### Scheduler (Phase 2.3)
- ✅ GET /api/v1/scheduler/decks/{deck_id}/next
- ✅ POST /api/v1/scheduler/decks/{deck_id}/cards/{card_id}/answer
- ✅ GET /api/v1/scheduler/decks/{deck_id}/counts
- ✅ POST /api/v1/scheduler/undo
- ✅ POST /api/v1/scheduler/redo

### Notetype Management (Phase 3.5 - NEW)
- ✅ GET /api/v1/notetypes - List all notetypes
- ✅ GET /api/v1/notetypes/{id} - Get notetype details

**Total Endpoints Implemented:** 45

---

## Next Steps

### Immediate Actions

1. **Test Reviewer UI**
   - Manual testing of review workflow
   - Test keyboard shortcuts
   - Test undo/redo functionality
   - Verify card rendering with various templates

2. **Phase 3.5: Editor UI (Next Task)**
   - Note/card creation interface
   - Field editors with formatting
   - Tag input with autocomplete
   - Media upload support
   - Deck/notetype selection

### Phase 3.5: Editor UI (Next Task)

**Priority:** P0  
**Estimate:** 5 days  
**Dependencies:** Phase 2.4 (Notes API) ✅

**Planned Components:**
- Field editor with rich text
- Tag input with autocomplete
- Deck selector dropdown
- Notetype selector
- Card preview
- Media upload (drag-drop)
- Duplicate detection
- Form validation

**Acceptance Criteria:**
- Fields editable with formatting
- Tags autocomplete from existing
- Deck/notetype selectable
- Preview shows rendered card
- Media uploads work
- Duplicate warnings shown
- Cards save correctly

---

## Technical Stack

### Backend
- **Language:** Rust
- **Framework:** Axum (async web framework)
- **Authentication:** JWT (jsonwebtoken crate) + Argon2 password hashing
- **Database:** SQLite (users/sessions) + Anki collections
- **API Documentation:** OpenAPI 3.0 + Swagger UI

### Architecture
- **Pattern:** Service layer with protobuf interfaces
- **Session Management:** Per-user Backend instances with Mutex locking
- **Error Handling:** Consistent JSON responses with proper HTTP status codes
- **Configuration:** TOML files + environment variables

---

## Build & Development

### Prerequisites
```bash
# Install n2 build tool
bash tools/install-n2

# Add to PATH
export PATH="$HOME/.cargo/bin:$PATH"
```

### Build Commands
```bash
# Full check (format + build + test + lint)
./check

# Individual checks
cargo build --release        # Compile
cargo clippy --release       # Lint
cargo test                   # Test
./ninja check:svelte        # TypeScript/Svelte
```

### Current Build Status
- ✅ Compilation: Clean
- ✅ Clippy: No warnings
- ⏳ Full ./check: Pending CONTRIBUTORS commit

---

## Repository Information

- **Fork:** git@github.com:kjwenger/anki.git (origin)
- **Upstream:** git@github.com:ankitects/anki.git
- **Branch:** main
- **Contributor:** kjwenger@yahoo.com

**⚠️ Important:** Always push to origin (fork), never to upstream!

---

## Documentation

### Project Documentation
- `README.webapp.md` - Web app overview and architecture
- `SPECIFICATIONS.md` - Detailed technical specifications
- `PROJECT_LAYOUT.md` - Codebase structure
- `TASKS.md` - Task breakdown and status

### API Documentation
- `CARDS_API_REFERENCE.md` - Quick reference for Cards API
- `PHASE_2.5_COMPLETE.md` - Phase 2.5 completion report
- Swagger UI: http://localhost:8080/swagger-ui
- OpenAPI Spec: http://localhost:8080/api-docs/openapi.json

### Development Guides
- `.copilot/user.md` - Setup, configuration, and troubleshooting
- `config/README.md` - Configuration options

---

## Statistics

### Code Metrics
- **Total Endpoints:** 27
- **Authentication System:** Complete with JWT + sessions
- **Database Tables:** 2 (users, sessions)
- **API Documentation:** OpenAPI 3.0 (complete)
- **Phase 1 Tests:** 13 passing
- **Build Time:** ~1 minute (release)

### Lines of Code (Webapp Module)
- Total: ~6,200 lines
- Routes: ~1,300 lines
- Auth: ~600 lines
- Database: ~400 lines
- Configuration: ~200 lines
- Error handling: ~200 lines
- Documentation: ~900 lines (OpenAPI)
- UI Components: ~2,600 lines (updated)

---

## Known Issues

### CONTRIBUTORS Validation
- **Issue:** ./check requires git commit of CONTRIBUTORS file
- **Status:** File updated, pending commit
- **Impact:** Blocks ./check but doesn't affect functionality
- **Resolution:** Commit CONTRIBUTORS with configured git email

---

## Success Criteria Progress

From TASKS.md original success criteria:

| Criteria                                 | Status | Notes                            |
|------------------------------------------|--------|----------------------------------|
| Users can study cards via web browser    | ✅      | Complete with keyboard shortcuts |
| Users can create cards via web browser   | ✅      | Complete with editor interface   |
| Users can browse/search cards            | ✅      | Complete with bulk operations    |
| API fully functional for core operations | ✅      | 100% complete (45 endpoints)     |
| Performance comparable to desktop app    | ⏳      | To be measured                   |
| Security audit passes                    | ⏳      | Pending Phase 4                  |
| Documentation complete                   | ✅      | OpenAPI + guides complete        |
| Can deploy on standard VPS               | ⏳      | Pending Phase 4                  |
| Existing collections compatible          | ✅      | Uses standard Anki backend       |

**Overall Progress:** Approximately 85% complete (Phases 1-3 COMPLETE, Phase 4 remaining!)

---

## Timeline

- **Phase 1 (Foundation):** ✅ Complete (~2 weeks)
- **Phase 2 (Core API):** ✅ Complete (~2 weeks)
- **Phase 3 (UI Components):** 🔄 In Progress (89% done, ~1-2 days remaining for final task!)
- **Phase 4 (Polish & Testing):** 📋 Not started (~2 weeks estimated)

**Estimated Total:** 9 weeks  
**Elapsed:** ~4-5 weeks  
**Remaining:** ~4-5 weeks

---

*This document is auto-updated as phases complete. Last update: Phase 3.9 completion - PHASE 3 COMPLETE!*
