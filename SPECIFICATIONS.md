# Anki Web App - Technical Specifications

## Overview

This document specifies the architecture for converting Anki's desktop application into a headless web application with a RESTful API. The implementation will reuse existing components while creating a modern web interface.

## Goals

1. **Headless Operation**: Run Anki without PyQt, purely as a web service
2. **REST API**: Expose all Anki functionality via clean REST endpoints
3. **Web UI**: Modern Svelte-based interface accessible via browser
4. **Multi-User**: Support multiple users with separate collections
5. **Compatibility**: Maintain compatibility with existing .anki2 files
6. **Security**: Proper authentication and authorization

## Architecture

### High-Level Components

```
┌─────────────────────────────────────────────────────────┐
│                     Anki Web Application                │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌───────────────────────────────────────────────────┐  │
│  │           Web Frontend (SvelteKit)                │  │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐   │  │
│  │  │  Reviewer  │  │   Decks    │  │   Editor   │   │  │
│  │  └────────────┘  └────────────┘  └────────────┘   │  │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐   │  │
│  │  │   Stats    │  │  Browser   │  │  Settings  │   │  │
│  │  └────────────┘  └────────────┘  └────────────┘   │  │
│  └──────────────────────┬────────────────────────────┘  │
│                         │ HTTP/JSON                     │
│  ┌──────────────────────▼────────────────────────────┐  │
│  │              REST API Server (Axum)               │  │
│  │  ┌────────────────────────────────────────────┐   │  │
│  │  │ Authentication (JWT)                       │   │  │
│  │  │ Session Management                         │   │  │
│  │  │ Multi-user Collection Routing              │   │  │
│  │  └────────────────────────────────────────────┘   │  │
│  │  ┌────────────────────────────────────────────┐   │  │
│  │  │ REST Endpoints                             │   │  │
│  │  │  - /api/v1/collections/*                   │   │  │
│  │  │  - /api/v1/decks/*                         │   │  │
│  │  │  - /api/v1/cards/*                         │   │  │
│  │  │  - /api/v1/notes/*                         │   │  │
│  │  │  - /api/v1/scheduler/*                     │   │  │
│  │  │  - /api/v1/search/*                        │   │  │
│  │  │  - /api/v1/media/*                         │   │  │
│  │  └────────────────────────────────────────────┘   │  │
│  └──────────────────────┬────────────────────────────┘  │
│                         │ Protobuf Messages             │
│  ┌──────────────────────▼────────────────────────────┐  │
│  │         Anki Backend (Existing Rust Core)         │  │
│  │  - rslib/src/backend                              │  │
│  │  - Collection management                          │  │
│  │  - Scheduler (FSRS)                               │  │
│  │  - SQLite database                                │  │
│  │  - Search engine                                  │  │
│  │  - Media management                               │  │
│  └───────────────────────────────────────────────────┘  │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

## Component Specifications

### 1. REST API Server

**Technology**: Rust + Axum web framework (already in dependencies)

**Location**: `rslib/webapp/`

**Key Features**:

- JWT-based authentication
- Per-user collection management
- Protobuf ↔ JSON conversion
- WebSocket support for real-time updates
- CORS configuration for web clients

**Server Configuration**:

```rust
struct WebAppConfig {
    host: String,              // Default: "127.0.0.1"
    port: u16,                 // Default: 8080
    data_dir: PathBuf,         // Base directory for user collections
    jwt_secret: String,        // JWT signing secret
    session_timeout: Duration, // Default: 24 hours
    max_upload_size: usize,    // Default: 100MB
}
```

### 2. API Endpoints

#### Authentication

```
POST   /api/v1/auth/register        - Register new user
POST   /api/v1/auth/login           - Login and get JWT token
POST   /api/v1/auth/logout          - Invalidate session
GET    /api/v1/auth/profile         - Get current user info
PUT    /api/v1/auth/profile         - Update user profile
POST   /api/v1/auth/refresh         - Refresh JWT token
```

#### Collections

```
GET    /api/v1/collections                      - List user's collections
POST   /api/v1/collections                      - Create new collection
GET    /api/v1/collections/{id}                 - Get collection info
DELETE /api/v1/collections/{id}                 - Delete collection
POST   /api/v1/collections/{id}/open            - Open collection (create session)
POST   /api/v1/collections/{id}/close           - Close collection
POST   /api/v1/collections/{id}/backup          - Create backup
GET    /api/v1/collections/{id}/check           - Run integrity check
```

#### Decks

```
GET    /api/v1/decks                            - List all decks (tree)
POST   /api/v1/decks                            - Create deck
GET    /api/v1/decks/{id}                       - Get deck details
PUT    /api/v1/decks/{id}                       - Update deck
DELETE /api/v1/decks/{id}                       - Delete deck
POST   /api/v1/decks/{id}/rename                - Rename deck
GET    /api/v1/decks/{id}/stats                 - Get deck statistics
POST   /api/v1/decks/{id}/set-current           - Set as current deck
GET    /api/v1/decks/{id}/config                - Get deck configuration
PUT    /api/v1/decks/{id}/config                - Update deck configuration
```

#### Cards

```
GET    /api/v1/cards/{id}                       - Get card
PUT    /api/v1/cards/{id}                       - Update card
DELETE /api/v1/cards/{id}                       - Delete card
POST   /api/v1/cards/{id}/flag                  - Set flag
POST   /api/v1/cards/{id}/suspend               - Suspend card
POST   /api/v1/cards/{id}/unsuspend             - Unsuspend card
POST   /api/v1/cards/{id}/bury                  - Bury card
GET    /api/v1/cards                            - Batch get cards
PUT    /api/v1/cards                            - Batch update cards
DELETE /api/v1/cards                            - Batch delete cards
```

#### Notes

```
GET    /api/v1/notes/{id}                       - Get note
POST   /api/v1/notes                            - Create note
PUT    /api/v1/notes/{id}                       - Update note
DELETE /api/v1/notes/{id}                       - Delete note
GET    /api/v1/notes/{id}/cards                 - Get note's cards
POST   /api/v1/notes/batch                      - Create multiple notes
PUT    /api/v1/notes/batch                      - Update multiple notes
POST   /api/v1/notes/{id}/tags                  - Add tags
DELETE /api/v1/notes/{id}/tags                  - Remove tags
```

#### Scheduler

```
GET    /api/v1/scheduler/next                   - Get next card(s) to review
POST   /api/v1/scheduler/answer                 - Answer card
GET    /api/v1/scheduler/counts                 - Get review counts
GET    /api/v1/scheduler/congrats               - Get completion info
POST   /api/v1/scheduler/custom-study           - Create custom study session
POST   /api/v1/scheduler/bury-deck              - Bury deck
POST   /api/v1/scheduler/unbury                 - Unbury cards
POST   /api/v1/scheduler/empty-filtered         - Empty filtered deck
POST   /api/v1/scheduler/rebuild-filtered       - Rebuild filtered deck
GET    /api/v1/scheduler/states/{card_id}       - Get scheduling states
POST   /api/v1/scheduler/set-due-date           - Set custom due date
```

#### Search

```
POST   /api/v1/search/cards                     - Search cards
POST   /api/v1/search/notes                     - Search notes
POST   /api/v1/search/find-replace              - Find and replace
```

#### Tags

```
GET    /api/v1/tags                             - List all tags
POST   /api/v1/tags                             - Create tag
PUT    /api/v1/tags/{name}                      - Rename tag
DELETE /api/v1/tags/{name}                      - Delete tag
POST   /api/v1/tags/clear-unused                - Clear unused tags
```

#### Statistics

```
GET    /api/v1/stats/deck/{id}                  - Deck statistics
GET    /api/v1/stats/collection                 - Collection statistics
GET    /api/v1/stats/graphs                     - Graph data
GET    /api/v1/stats/studied-today              - Today's study summary
```

#### Media

```
GET    /api/v1/media/{filename}                 - Get media file
POST   /api/v1/media                            - Upload media file
DELETE /api/v1/media/{filename}                 - Delete media file
GET    /api/v1/media/check                      - Check media integrity
POST   /api/v1/media/sync                       - Sync media
```

#### Import/Export

```
POST   /api/v1/import/apkg                      - Import .apkg file
POST   /api/v1/import/csv                       - Import CSV
POST   /api/v1/import/collection                - Import collection
POST   /api/v1/export/apkg                      - Export deck as .apkg
POST   /api/v1/export/collection                - Export collection
GET    /api/v1/export/{job_id}/status           - Get export job status
GET    /api/v1/export/{job_id}/download         - Download export file
```

#### Notetypes

```
GET    /api/v1/notetypes                        - List notetypes
POST   /api/v1/notetypes                        - Create notetype
GET    /api/v1/notetypes/{id}                   - Get notetype
PUT    /api/v1/notetypes/{id}                   - Update notetype
DELETE /api/v1/notetypes/{id}                   - Delete notetype
POST   /api/v1/notetypes/{id}/templates         - Add template
PUT    /api/v1/notetypes/{id}/templates/{idx}   - Update template
DELETE /api/v1/notetypes/{id}/templates/{idx}   - Delete template
```

### 3. Web Frontend

**Technology**: SvelteKit 2 (already in use)

**Location**: `ts/routes/`

**New Pages Required**:

#### Deck Browser (`ts/routes/decks/+page.svelte`)

- Display deck tree with expand/collapse
- Show new/learning/review counts
- Quick study button
- Deck options button
- Search/filter decks

#### Reviewer (`ts/routes/review/+page.svelte`)

- Card question/answer display
- Answer buttons (Again/Hard/Good/Easy)
- Keyboard shortcuts
- Progress indicator
- Audio playback
- Edit card button
- Flag/suspend/bury options
- Undo/redo support

#### Editor (`ts/routes/editor/+page.svelte`)

- Rich text field editing
- Tag input with autocomplete
- Deck selector
- Notetype selector
- Card template preview
- Media upload (drag-drop)
- Duplicate detection

#### Card Browser (`ts/routes/browse/+page.svelte`)

- Search interface
- Card list with columns
- Column customization
- Multi-select
- Bulk operations
- Filter sidebar
- Preview pane

#### Settings (`ts/routes/settings/+page.svelte`)

- User preferences
- Collection settings
- Scheduling options
- Appearance settings

**Existing Pages** (already implemented):

- ✅ Statistics/Graphs
- ✅ Deck Options
- ✅ Import CSV
- ✅ Import Package
- ✅ Image Occlusion
- ✅ Congrats Screen

### 4. Authentication & Multi-User Support

**User Model**:

```rust
struct User {
    id: Uuid,
    username: String,
    email: String,
    password_hash: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
```

**Session Model**:

```rust
struct Session {
    user_id: Uuid,
    collection_id: Option<Uuid>,
    backend: Arc<Backend>,
    last_activity: DateTime<Utc>,
}
```

**JWT Claims**:

```rust
struct Claims {
    sub: Uuid,           // user_id
    exp: i64,            // expiration
    iat: i64,            // issued at
    username: String,
}
```

### 5. Data Storage

**Directory Structure**:

```
{data_dir}/
├── users/
│   ├── {user_id}/
│   │   ├── collections/
│   │   │   ├── {collection_name}.anki2
│   │   │   └── {collection_name}.media/
│   │   └── backups/
│   └── ...
├── config/
│   └── server.toml
└── logs/
    └── webapp.log
```

**Database**: SQLite for user management

```sql
CREATE TABLE users (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE TABLE sessions (
    token TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    expires_at INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
```

### 6. API Response Format

**Success Response**:

```json
{
    "success": true,
    "data": {/* response data */}
}
```

**Error Response**:

```json
{
    "success": false,
    "error": {
        "code": "INVALID_INPUT",
        "message": "Card not found",
        "details": {/* optional additional info */}
    }
}
```

**List Response with Pagination**:

```json
{
    "success": true,
    "data": {
        "items": [/* array of items */],
        "pagination": {
            "total": 100,
            "page": 1,
            "per_page": 20,
            "total_pages": 5
        }
    }
}
```

### 7. WebSocket Support (Optional)

For real-time updates (progress, sync status):

```
WS /api/v1/ws
```

**Message Format**:

```json
{
    "type": "progress",
    "data": {
        "operation": "import",
        "progress": 0.5,
        "message": "Importing cards..."
    }
}
```

## Security Considerations

1. **Authentication**: JWT with secure secret, HTTP-only cookies option
2. **Authorization**: Per-user collection isolation
3. **Rate Limiting**: Prevent brute force attacks
4. **Input Validation**: Validate all API inputs
5. **SQL Injection**: Use parameterized queries
6. **XSS Prevention**: Sanitize user content in frontend
7. **CSRF Protection**: Token-based protection for state-changing operations
8. **File Upload**: Validate file types, size limits, virus scanning
9. **HTTPS**: Enforce HTTPS in production
10. **Password Storage**: bcrypt/argon2 hashing

## Configuration

**Server Configuration** (`config/server.toml`):

```toml
[server]
host = "127.0.0.1"
port = 8080
workers = 4

[auth]
jwt_secret = "change-this-secret"
session_timeout_hours = 24
password_min_length = 8

[collections]
data_dir = "./data"
max_collections_per_user = 5
backup_retention_days = 30

[media]
max_file_size_mb = 100
allowed_extensions = ["jpg", "png", "gif", "mp3", "mp4", "webm"]

[limits]
max_request_size_mb = 100
rate_limit_requests_per_minute = 60

[logging]
level = "info"
file = "logs/webapp.log"
```

## Performance Considerations

1. **Connection Pooling**: Reuse database connections
2. **Caching**: Cache frequently accessed data (deck trees, notetypes)
3. **Lazy Loading**: Load card content on demand
4. **Pagination**: Paginate large result sets
5. **Compression**: Gzip response compression
6. **Static Assets**: CDN for frontend assets
7. **Database Indexing**: Proper indexes on search fields

## Compatibility

- **File Format**: Uses existing `.anki2` (SQLite) format
- **Media**: Compatible with existing media storage
- **Sync**: Can sync with AnkiWeb (if sync server implemented)
- **Import/Export**: Supports `.apkg` format
- **Desktop Interop**: Can open same collection (not simultaneously)

## Deployment Options

1. **Local**: Run on localhost for single-user access
2. **LAN**: Run on local network for household access
3. **VPS**: Deploy on cloud server for remote access
4. **Docker**: Containerized deployment
5. **Self-hosted**: Full control over data

## Migration Path

For existing desktop users:

1. **Install**: Install web app server
2. **Import**: Point to existing collection path
3. **Access**: Open browser to `http://localhost:8080`
4. **Desktop**: Can still use desktop app (not simultaneously)

## Future Enhancements

1. **Collaborative Decks**: Share decks between users
2. **Real-time Sync**: Live collaboration
3. **Mobile API**: Optimized endpoints for mobile apps
4. **Plugin System**: Web-based add-ons
5. **Advanced Analytics**: Enhanced statistics
6. **Social Features**: Study groups, leaderboards
7. **AI Integration**: Smart scheduling suggestions
