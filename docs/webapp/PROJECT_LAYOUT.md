# Anki Web App - Suggested Project Layout

This document describes the recommended directory structure for the web app implementation, harmonizing with the existing Anki codebase.

## Overview

The web app components are organized into three main areas:

1. **Backend (Rust)**: `rslib/webapp/` - REST API server
2. **Frontend (TypeScript/Svelte)**: `ts/routes/webapp/` - Web UI
3. **Configuration**: `config/` - Server and deployment configs

## Complete Directory Structure

```
anki/
├── rslib/
│   ├── webapp/                          # NEW: Web app backend
│   │   ├── Cargo.toml                   # Webapp crate config
│   │   ├── build.rs                     # Build script
│   │   ├── README.md                    # Webapp-specific docs
│   │   │
│   │   ├── src/
│   │   │   ├── main.rs                  # Server entry point
│   │   │   ├── lib.rs                   # Library exports
│   │   │   ├── config.rs                # Configuration management
│   │   │   ├── error.rs                 # Error types
│   │   │   │
│   │   │   ├── server/                  # Axum server setup
│   │   │   │   ├── mod.rs
│   │   │   │   ├── state.rs             # Shared server state
│   │   │   │   ├── middleware.rs        # Custom middleware
│   │   │   │   └── router.rs            # Route registration
│   │   │   │
│   │   │   ├── auth/                    # Authentication
│   │   │   │   ├── mod.rs
│   │   │   │   ├── jwt.rs               # JWT handling
│   │   │   │   ├── password.rs          # Password hashing
│   │   │   │   └── middleware.rs        # Auth middleware
│   │   │   │
│   │   │   ├── db/                      # User database
│   │   │   │   ├── mod.rs
│   │   │   │   ├── schema.sql           # SQLite schema
│   │   │   │   ├── users.rs             # User CRUD
│   │   │   │   ├── sessions.rs          # Session management
│   │   │   │   └── migrations.rs        # Schema migrations
│   │   │   │
│   │   │   ├── session/                 # Session management
│   │   │   │   ├── mod.rs
│   │   │   │   ├── store.rs             # Session store
│   │   │   │   ├── manager.rs           # Backend instance manager
│   │   │   │   └── middleware.rs        # Session middleware
│   │   │   │
│   │   │   ├── routes/                  # API route handlers
│   │   │   │   ├── mod.rs
│   │   │   │   ├── auth.rs              # /api/v1/auth/*
│   │   │   │   ├── collections.rs       # /api/v1/collections/*
│   │   │   │   ├── decks.rs             # /api/v1/decks/*
│   │   │   │   ├── cards.rs             # /api/v1/cards/*
│   │   │   │   ├── notes.rs             # /api/v1/notes/*
│   │   │   │   ├── scheduler.rs         # /api/v1/scheduler/*
│   │   │   │   ├── search.rs            # /api/v1/search/*
│   │   │   │   ├── tags.rs              # /api/v1/tags/*
│   │   │   │   ├── media.rs             # /api/v1/media/*
│   │   │   │   ├── stats.rs             # /api/v1/stats/*
│   │   │   │   ├── notetypes.rs         # /api/v1/notetypes/*
│   │   │   │   └── import_export.rs     # /api/v1/import/*, /api/v1/export/*
│   │   │   │
│   │   │   ├── handlers/                # Request handlers (business logic)
│   │   │   │   ├── mod.rs
│   │   │   │   ├── collections.rs
│   │   │   │   ├── decks.rs
│   │   │   │   ├── cards.rs
│   │   │   │   ├── notes.rs
│   │   │   │   ├── scheduler.rs
│   │   │   │   ├── search.rs
│   │   │   │   ├── tags.rs
│   │   │   │   ├── media.rs
│   │   │   │   ├── stats.rs
│   │   │   │   └── notetypes.rs
│   │   │   │
│   │   │   ├── models/                  # Request/response models
│   │   │   │   ├── mod.rs
│   │   │   │   ├── auth.rs              # Login, register, etc.
│   │   │   │   ├── collection.rs        # Collection models
│   │   │   │   ├── deck.rs              # Deck models
│   │   │   │   ├── card.rs              # Card models
│   │   │   │   ├── note.rs              # Note models
│   │   │   │   ├── scheduler.rs         # Scheduler models
│   │   │   │   └── response.rs          # Common response types
│   │   │   │
│   │   │   ├── utils/                   # Utility functions
│   │   │   │   ├── mod.rs
│   │   │   │   ├── pagination.rs        # Pagination helpers
│   │   │   │   ├── validation.rs        # Input validation
│   │   │   │   ├── serialization.rs     # Protobuf ↔ JSON
│   │   │   │   └── file_handling.rs     # File upload/download
│   │   │   │
│   │   │   └── ws/                      # WebSocket support (optional)
│   │   │       ├── mod.rs
│   │   │       ├── handler.rs
│   │   │       └── messages.rs
│   │   │
│   │   ├── tests/                       # Integration tests
│   │   │   ├── common/
│   │   │   │   └── mod.rs               # Test helpers
│   │   │   ├── auth_test.rs
│   │   │   ├── collections_test.rs
│   │   │   ├── scheduler_test.rs
│   │   │   └── api_test.rs
│   │   │
│   │   └── benches/                     # Benchmarks
│   │       ├── api.rs
│   │       └── scheduler.rs
│   │
│   ├── src/                             # Existing Anki core
│   ├── ...                              # Other existing modules
│
├── ts/
│   ├── routes/
│   │   ├── webapp/                      # NEW: Web app routes
│   │   │   ├── +layout.svelte          # Root layout
│   │   │   ├── +layout.ts              # Root layout data
│   │   │   ├── +page.svelte            # Landing/redirect page
│   │   │   │
│   │   │   ├── auth/                   # Authentication pages
│   │   │   │   ├── login/
│   │   │   │   │   └── +page.svelte
│   │   │   │   ├── register/
│   │   │   │   │   └── +page.svelte
│   │   │   │   └── profile/
│   │   │   │       └── +page.svelte
│   │   │   │
│   │   │   ├── collections/            # Collection management
│   │   │   │   ├── +page.svelte        # List collections
│   │   │   │   └── +page.ts
│   │   │   │
│   │   │   ├── decks/                  # Deck browser
│   │   │   │   ├── +page.svelte        # Deck list/tree
│   │   │   │   └── +page.ts
│   │   │   │
│   │   │   ├── review/                 # Reviewer
│   │   │   │   ├── +page.svelte        # Review interface
│   │   │   │   ├── +page.ts
│   │   │   │   └── [deckId]/           # Deck-specific review
│   │   │   │       └── +page.svelte
│   │   │   │
│   │   │   ├── editor/                 # Card editor
│   │   │   │   ├── +page.svelte        # Add card
│   │   │   │   ├── +page.ts
│   │   │   │   └── [noteId]/           # Edit existing
│   │   │   │       └── +page.svelte
│   │   │   │
│   │   │   ├── browse/                 # Card browser
│   │   │   │   ├── +page.svelte
│   │   │   │   └── +page.ts
│   │   │   │
│   │   │   ├── stats/                  # Statistics
│   │   │   │   ├── +page.svelte
│   │   │   │   └── +page.ts
│   │   │   │
│   │   │   └── settings/               # Settings
│   │   │       ├── +page.svelte
│   │   │       └── +page.ts
│   │   │
│   │   ├── deck-options/               # Existing (reuse)
│   │   ├── graphs/                     # Existing (reuse)
│   │   ├── congrats/                   # Existing (reuse)
│   │   └── ...                         # Other existing routes
│   │
│   ├── lib/
│   │   ├── webapp/                     # NEW: Web app components & utilities
│   │   │   ├── components/             # Reusable components
│   │   │   │   ├── auth/
│   │   │   │   │   ├── LoginForm.svelte
│   │   │   │   │   ├── RegisterForm.svelte
│   │   │   │   │   └── ProtectedRoute.svelte
│   │   │   │   │
│   │   │   │   ├── layout/
│   │   │   │   │   ├── NavBar.svelte
│   │   │   │   │   ├── Sidebar.svelte
│   │   │   │   │   ├── UserMenu.svelte
│   │   │   │   │   └── Breadcrumbs.svelte
│   │   │   │   │
│   │   │   │   ├── collections/
│   │   │   │   │   ├── CollectionList.svelte
│   │   │   │   │   ├── CollectionCard.svelte
│   │   │   │   │   └── CreateCollectionDialog.svelte
│   │   │   │   │
│   │   │   │   ├── decks/
│   │   │   │   │   ├── DeckTree.svelte
│   │   │   │   │   ├── DeckNode.svelte
│   │   │   │   │   ├── DeckDialog.svelte
│   │   │   │   │   └── DeckStats.svelte
│   │   │   │   │
│   │   │   │   ├── reviewer/
│   │   │   │   │   ├── CardDisplay.svelte
│   │   │   │   │   ├── AnswerButtons.svelte
│   │   │   │   │   ├── ReviewProgress.svelte
│   │   │   │   │   └── CardActions.svelte
│   │   │   │   │
│   │   │   │   ├── editor/
│   │   │   │   │   ├── FieldEditor.svelte
│   │   │   │   │   ├── TagInput.svelte
│   │   │   │   │   ├── DeckSelector.svelte
│   │   │   │   │   ├── NotetypeSelector.svelte
│   │   │   │   │   ├── CardPreview.svelte
│   │   │   │   │   └── MediaUpload.svelte
│   │   │   │   │
│   │   │   │   ├── browser/
│   │   │   │   │   ├── SearchBar.svelte
│   │   │   │   │   ├── CardTable.svelte
│   │   │   │   │   ├── FilterSidebar.svelte
│   │   │   │   │   ├── PreviewPane.svelte
│   │   │   │   │   └── BulkActions.svelte
│   │   │   │   │
│   │   │   │   ├── stats/
│   │   │   │   │   ├── StatsOverview.svelte
│   │   │   │   │   ├── CalendarHeatmap.svelte
│   │   │   │   │   └── GraphSelector.svelte
│   │   │   │   │
│   │   │   │   └── common/
│   │   │   │       ├── Button.svelte
│   │   │   │       ├── Input.svelte
│   │   │   │       ├── Modal.svelte
│   │   │   │       ├── Toast.svelte
│   │   │   │       ├── Spinner.svelte
│   │   │   │       └── Pagination.svelte
│   │   │   │
│   │   │   ├── stores/                 # Svelte stores
│   │   │   │   ├── auth.ts             # Authentication state
│   │   │   │   ├── collection.ts       # Active collection
│   │   │   │   ├── reviewer.ts         # Review session state
│   │   │   │   ├── editor.ts           # Editor state
│   │   │   │   ├── browser.ts          # Browser state
│   │   │   │   └── toast.ts            # Toast notifications
│   │   │   │
│   │   │   ├── api/                    # API client
│   │   │   │   ├── client.ts           # Base HTTP client
│   │   │   │   ├── auth.ts             # Auth endpoints
│   │   │   │   ├── collections.ts      # Collection endpoints
│   │   │   │   ├── decks.ts            # Deck endpoints
│   │   │   │   ├── cards.ts            # Card endpoints
│   │   │   │   ├── notes.ts            # Note endpoints
│   │   │   │   ├── scheduler.ts        # Scheduler endpoints
│   │   │   │   ├── search.ts           # Search endpoints
│   │   │   │   ├── tags.ts             # Tag endpoints
│   │   │   │   ├── media.ts            # Media endpoints
│   │   │   │   ├── stats.ts            # Stats endpoints
│   │   │   │   └── types.ts            # TypeScript types
│   │   │   │
│   │   │   ├── utils/                  # Utility functions
│   │   │   │   ├── keyboard.ts         # Keyboard shortcuts
│   │   │   │   ├── formatting.ts       # Text formatting
│   │   │   │   ├── validation.ts       # Input validation
│   │   │   │   ├── time.ts             # Time utilities
│   │   │   │   └── storage.ts          # Local storage
│   │   │   │
│   │   │   └── styles/                 # Shared styles
│   │   │       ├── variables.scss      # CSS variables
│   │   │       ├── mixins.scss         # SCSS mixins
│   │   │       └── global.scss         # Global styles
│   │   │
│   │   ├── components/                 # Existing shared components
│   │   ├── ...                         # Other existing lib folders
│   │
│   └── tests/
│       └── webapp/                     # NEW: Frontend tests
│           ├── unit/
│           │   ├── components/
│           │   └── stores/
│           └── e2e/
│               ├── auth.spec.ts
│               ├── study-flow.spec.ts
│               └── editor.spec.ts
│
├── config/                             # NEW: Configuration files
│   ├── server.toml.example             # Example server config
│   └── development.toml                # Dev config
│
├── deploy/                             # NEW: Deployment files
│   ├── Dockerfile                      # Docker image
│   ├── docker-compose.yml              # Docker compose setup
│   ├── anki-webapp.service             # Systemd service
│   ├── nginx.conf.example              # Nginx reverse proxy config
│   └── install.sh                      # Installation script
│
├── docs/
│   └── webapp/                         # NEW: Web app documentation
│       ├── api.yaml                    # OpenAPI spec
│       ├── user-guide.md               # User documentation
│       ├── admin-guide.md              # Admin documentation
│       ├── deployment.md               # Deployment guide
│       └── development.md              # Development guide
│
├── SPECIFICATIONS.md                   # NEW: Technical specifications
├── TASKS.md                            # NEW: Implementation tasks
├── README.webapp.md                    # NEW: Web app README
│
├── Cargo.toml                          # Add webapp to workspace
├── package.json                        # Existing
├── ...                                 # Other existing files
```

## Key Integration Points

### 1. Cargo Workspace Integration

Update `Cargo.toml` to include webapp:

```toml
[workspace]
members = [
  # ... existing members
  "rslib/webapp",
]
```

### 2. Build System Integration

The webapp should integrate with the existing Ninja build system:

```python
# build/configure/src/web.py (new file)
def setup_webapp():
    """Configure webapp build targets"""
    # Add webapp build targets to ninja
```

### 3. TypeScript API Generation

Leverage existing protobuf → TypeScript generation:

```
rslib/proto/*.proto
    ↓ (existing build process)
ts/lib/generated/backend.ts
    ↓ (new wrapper)
ts/lib/webapp/api/client.ts
```

### 4. Shared Components

Reuse existing Svelte components where possible:

- Deck options UI (already exists)
- Statistics graphs (already exists)
- Import wizards (already exist)

## Development Workflow

### Backend Development

```bash
# Run webapp server in development mode
cd rslib/webapp
cargo run

# Run tests
cargo test

# Run with auto-reload
cargo watch -x run
```

### Frontend Development

```bash
# Run SvelteKit dev server
cd ts
npm run dev

# Points to backend at http://localhost:8080
```

### Full Stack Development

```bash
# Terminal 1: Backend
cd rslib/webapp && cargo watch -x run

# Terminal 2: Frontend
cd ts && npm run dev

# Open browser to http://localhost:5173
```

## Build Artifacts

```
out/
├── webapp/
│   ├── anki-webapp              # Compiled binary
│   ├── static/                  # Built frontend
│   │   ├── _app/
│   │   ├── index.html
│   │   └── ...
│   └── config/
│       └── server.toml
```

## Configuration Hierarchy

The webapp looks for configuration in this order:

1. Environment variables (`ANKI_WEBAPP_*`)
2. Config file (`config/server.toml`)
3. Default values (hardcoded)

## Database Files

```
{data_dir}/
├── webapp.db                    # User/session database
├── webapp.db-wal                # SQLite WAL file
└── users/
    └── {user_id}/
        └── collections/
            ├── collection.anki2
            └── collection.media/
```

## Naming Conventions

### Rust

- Modules: `snake_case`
- Types: `PascalCase`
- Functions: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`

### TypeScript

- Components: `PascalCase.svelte`
- Functions: `camelCase`
- Types: `PascalCase`
- Constants: `SCREAMING_SNAKE_CASE`

### API Routes

- REST paths: `kebab-case`
- Query params: `snake_case`
- JSON keys: `camelCase` or `snake_case` (match protobuf)

## Testing Structure

```
# Backend tests
rslib/webapp/tests/
├── integration/          # API integration tests
├── unit/                # Unit tests (inline in src/)
└── common/              # Test helpers

# Frontend tests
ts/tests/webapp/
├── unit/                # Component tests
├── integration/         # Store/API tests
└── e2e/                 # End-to-end tests
```

## Notes

- **Coexistence**: The webapp code should not interfere with existing Qt desktop app
- **Sharing**: Reuse existing Rust backend, protobuf definitions, and some Svelte components
- **Independence**: Webapp can be built/tested independently
- **Optional**: Webapp is an optional feature, not required for desktop app
