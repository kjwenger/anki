# Anki Web App - Implementation Tasks

This document breaks down the implementation into manageable tasks with priorities and time estimates.

## Project Phases

### Phase 1: Foundation (Week 1-2)

Core infrastructure and authentication

### Phase 2: Core API (Week 3-4)

Essential REST endpoints

### Phase 3: UI Components (Week 5-7)

Web interface pages

### Phase 4: Polish & Testing (Week 8-9)

Testing, documentation, deployment

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
- [ ] PUT /api/v1/decks/{id} (update) - future
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

### 2.8 Tags API

**Priority**: P2\
**Estimate**: 1 day\
**Dependencies**: 2.1

- [ ] GET /api/v1/tags
- [ ] POST /api/v1/tags
- [ ] PUT /api/v1/tags/{name}
- [ ] DELETE /api/v1/tags/{name}
- [ ] POST /api/v1/tags/clear-unused

**Files to Create**:

- `rslib/webapp/src/routes/tags.rs`
- `rslib/webapp/src/handlers/tags.rs`

**Acceptance Criteria**:

- Tags list correctly
- Rename updates all notes
- Delete works properly

---

### 2.9 Statistics API

**Priority**: P2\
**Estimate**: 1 day\
**Dependencies**: 2.1

- [ ] GET /api/v1/stats/deck/{id}
- [ ] GET /api/v1/stats/collection
- [ ] GET /api/v1/stats/graphs
- [ ] GET /api/v1/stats/studied-today

**Files to Create**:

- `rslib/webapp/src/routes/stats.rs`
- `rslib/webapp/src/handlers/stats.rs`

**Acceptance Criteria**:

- Stats match desktop app
- Graph data correct format
- Performance acceptable

---

## Phase 3: UI Components (3 weeks)

### 3.1 Authentication UI

**Priority**: P0\
**Estimate**: 2 days\
**Dependencies**: 1.3

- [ ] Login page
- [ ] Registration page
- [ ] Password reset page (optional)
- [ ] Profile page
- [ ] Auth state management (stores)
- [ ] Protected route wrapper

**Files to Create**:

- `ts/routes/webapp/auth/login/+page.svelte`
- `ts/routes/webapp/auth/register/+page.svelte`
- `ts/routes/webapp/auth/profile/+page.svelte`
- `ts/lib/webapp/stores/auth.ts`

**Acceptance Criteria**:

- User can login/register
- JWT stored securely
- Auto-redirect on auth required
- Logout works

---

### 3.2 Collection Manager UI

**Priority**: P0\
**Estimate**: 1 day\
**Dependencies**: 2.1, 3.1

- [ ] Collection list page
- [ ] Create collection dialog
- [ ] Delete collection confirmation
- [ ] Collection selection
- [ ] Backup management

**Files to Create**:

- `ts/routes/webapp/collections/+page.svelte`
- `ts/lib/webapp/components/CollectionList.svelte`
- `ts/lib/webapp/components/CreateCollectionDialog.svelte`

**Acceptance Criteria**:

- User can create collections
- Can switch between collections
- Can delete collections

---

### 3.3 Deck Browser UI

**Priority**: P0\
**Estimate**: 3 days\
**Dependencies**: 2.2, 3.2

- [ ] Deck tree display component
- [ ] Expand/collapse functionality
- [ ] Study counts display
- [ ] Quick study button
- [ ] Deck options button
- [ ] Create/rename/delete deck dialogs
- [ ] Drag-drop to reparent (optional)

**Files to Create**:

- `ts/routes/webapp/decks/+page.svelte`
- `ts/lib/webapp/components/DeckTree.svelte`
- `ts/lib/webapp/components/DeckNode.svelte`
- `ts/lib/webapp/components/DeckDialog.svelte`

**Acceptance Criteria**:

- Deck tree displays correctly
- Counts update in real-time
- Deck management works
- Responsive design

---

### 3.4 Reviewer UI

**Priority**: P0\
**Estimate**: 5 days\
**Dependencies**: 2.3, 3.2

- [ ] Card display component
- [ ] Question/answer reveal
- [ ] Answer buttons (Again/Hard/Good/Easy)
- [ ] Keyboard shortcuts
- [ ] Progress indicator
- [ ] Audio playback integration
- [ ] Card actions menu (edit/flag/suspend/bury)
- [ ] Undo/redo support
- [ ] Study completion screen integration

**Files to Create**:

- `ts/routes/webapp/review/+page.svelte`
- `ts/lib/webapp/components/CardDisplay.svelte`
- `ts/lib/webapp/components/AnswerButtons.svelte`
- `ts/lib/webapp/components/ReviewProgress.svelte`
- `ts/lib/webapp/stores/reviewer.ts`

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

## Phase 4: Polish & Testing (2 weeks)

### 4.1 API Testing

**Priority**: P0\
**Estimate**: 3 days\
**Dependencies**: Phase 2 complete

- [ ] Write integration tests for auth
- [ ] Write integration tests for collections
- [ ] Write integration tests for scheduler
- [ ] Write integration tests for CRUD operations
- [ ] Write load tests
- [ ] Test error handling
- [ ] Test edge cases

**Files to Create**:

- `rslib/webapp/tests/integration/auth.rs`
- `rslib/webapp/tests/integration/collections.rs`
- `rslib/webapp/tests/integration/scheduler.rs`
- `rslib/webapp/benches/api.rs`

**Acceptance Criteria**:

- 80%+ code coverage
- All critical paths tested
- Load tests pass
- Edge cases handled

---

### 4.2 UI Testing

**Priority**: P1\
**Estimate**: 2 days\
**Dependencies**: Phase 3 complete

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

### 4.3 Documentation

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

### 4.4 Security Audit

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

### 4.5 Performance Optimization

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

### 4.6 Deployment Packaging

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

### Import/Export UI

**Priority**: P2\
**Estimate**: 3 days

- [ ] Import .apkg wizard
- [ ] Import CSV wizard
- [ ] Export deck dialog
- [ ] Export collection dialog
- [ ] Progress tracking

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

- [ ] Touch gesture support
- [ ] Mobile-specific layouts
- [ ] PWA manifest
- [ ] Offline support

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

| Phase                     | Duration    | Dependencies  |
| ------------------------- | ----------- | ------------- |
| Phase 1: Foundation       | 2 weeks     | None          |
| Phase 2: Core API         | 2 weeks     | Phase 1       |
| Phase 3: UI Components    | 3 weeks     | Phase 1, 2    |
| Phase 4: Polish & Testing | 2 weeks     | Phase 1, 2, 3 |
| **Total**                 | **9 weeks** |               |

## Resource Requirements

- **Developer(s)**: 1-2 full-stack developers
- **Skills Required**: Rust, TypeScript, Svelte, SQL, REST APIs
- **Infrastructure**: Development server, test environment

## Risk Assessment

| Risk                     | Likelihood | Impact   | Mitigation                       |
| ------------------------ | ---------- | -------- | -------------------------------- |
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
