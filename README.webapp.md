# Anki Web App - Setup and Usage Guide

A headless, multi-user web application for Anki with a modern browser-based interface and RESTful API.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Requirements](#requirements)
- [Quick Start](#quick-start)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [API Documentation](#api-documentation)
- [Development](#development)
- [Deployment](#deployment)
- [Troubleshooting](#troubleshooting)
- [FAQ](#faq)

## Overview

Anki Web App provides a complete Anki experience accessible through any modern web browser. It runs as a standalone server without requiring PyQt, making it ideal for:

- **Remote Access**: Study from any device with a browser
- **Multi-User**: Multiple users with separate collections
- **Headless Servers**: Run on servers without GUI
- **API Access**: Programmatic access to Anki functionality
- **Self-Hosting**: Full control over your data

### Architecture

```
┌─────────────────┐
│   Web Browser   │ ← Users access via HTTP/HTTPS
└────────┬────────┘
         │ REST API (JSON)
┌────────▼────────┐
│  Anki Web App   │ ← Rust server (port 8080)
│  - REST API     │
│  - Auth         │
│  - Sessions     │
└────────┬────────┘
         │ Protobuf
┌────────▼────────┐
│  Anki Backend   │ ← Existing Rust core
│  - SQLite DB    │
│  - Scheduler    │
│  - Media        │
└─────────────────┘
```

## Features

### Core Functionality

- ✅ **Study Cards**: Full reviewer with keyboard shortcuts
- ✅ **Add Cards**: Rich text editor with media support
- ✅ **Deck Management**: Create, organize, configure decks
- ✅ **Card Browser**: Search and edit cards in bulk
- ✅ **Statistics**: Comprehensive graphs and analytics
- ✅ **Import/Export**: Support for .apkg files
- ✅ **Media**: Upload and manage images, audio, video

### Web App Specific

- 🔐 **Multi-User**: Separate accounts and collections
- 🌐 **Remote Access**: Study from anywhere
- 📱 **Responsive**: Works on desktop, tablet, mobile
- 🔌 **REST API**: Programmatic access
- 🎨 **Modern UI**: Clean Svelte-based interface
- 🔒 **Secure**: JWT authentication, HTTPS support

## Requirements

### Server Requirements

- **OS**: Linux, macOS, or Windows
- **RAM**: 512 MB minimum, 1 GB recommended
- **Disk**: 100 MB for app + space for collections
- **Rust**: 1.80+ (for building from source)
- **Node.js**: 18+ (for building frontend)

### Client Requirements

- **Browser**: Chrome 77+, Firefox 78+, Safari 14.5+, Edge 79+
- **JavaScript**: Enabled
- **Connection**: HTTP/HTTPS access to server

## Quick Start

### Using Docker (Recommended)

```bash
# Pull and run the image
docker run -d \
  --name anki-webapp \
  -p 8080:8080 \
  -v ~/anki-data:/data \
  anki/webapp:latest

# Open browser to http://localhost:8080
```

### Using Binary Release

```bash
# Download latest release
wget https://github.com/ankitects/anki/releases/latest/download/anki-webapp-linux-x64.tar.gz

# Extract
tar xzf anki-webapp-linux-x64.tar.gz
cd anki-webapp

# Run
./anki-webapp

# Open browser to http://localhost:8080
```

### From Source

```bash
# Clone repository
git clone https://github.com/ankitects/anki.git
cd anki

# Build backend
cd rslib/webapp
cargo build --release

# Build frontend
cd ../../ts
npm install
npm run build

# Run
cd ../rslib/webapp
cargo run --release

# Open browser to http://localhost:8080
```

## Installation

### Method 1: Docker Compose (Production)

Create `docker-compose.yml`:

```yaml
version: '3.8'

services:
  anki-webapp:
    image: anki/webapp:latest
    container_name: anki-webapp
    restart: unless-stopped
    ports:
      - "8080:8080"
    volumes:
      - ./data:/data
      - ./config:/app/config
    environment:
      - ANKI_WEBAPP_HOST=0.0.0.0
      - ANKI_WEBAPP_PORT=8080
      - ANKI_WEBAPP_JWT_SECRET=your-secret-key-here-change-me
      - RUST_LOG=info
```

Start:

```bash
docker-compose up -d
```

### Method 2: Systemd Service (Linux)

Create `/etc/systemd/system/anki-webapp.service`:

```ini
[Unit]
Description=Anki Web App
After=network.target

[Service]
Type=simple
User=anki
WorkingDirectory=/opt/anki-webapp
ExecStart=/opt/anki-webapp/anki-webapp
Restart=on-failure
RestartSec=10

Environment="ANKI_WEBAPP_DATA_DIR=/var/lib/anki-webapp"
Environment="ANKI_WEBAPP_JWT_SECRET=your-secret-key-here"
Environment="RUST_LOG=info"

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl enable anki-webapp
sudo systemctl start anki-webapp
sudo systemctl status anki-webapp
```

### Method 3: Manual Installation

```bash
# Create installation directory
sudo mkdir -p /opt/anki-webapp
sudo chown $USER /opt/anki-webapp

# Copy binary and assets
cp -r target/release/anki-webapp /opt/anki-webapp/
cp -r static /opt/anki-webapp/
cp config/server.toml.example /opt/anki-webapp/config/server.toml

# Create data directory
mkdir -p ~/anki-webapp-data

# Edit configuration
nano /opt/anki-webapp/config/server.toml

# Run
cd /opt/anki-webapp
./anki-webapp
```

## Configuration

### Configuration File

Location: `config/server.toml`

```toml
[server]
# Network binding
host = "127.0.0.1" # Use "0.0.0.0" for all interfaces
port = 8080

# Worker threads
workers = 4

[auth]
# JWT secret (REQUIRED - change this!)
jwt_secret = "change-this-to-a-random-secret"

# Session timeout in hours
session_timeout_hours = 24

# Password requirements
password_min_length = 8
password_require_uppercase = true
password_require_number = true

[collections]
# Data directory for user collections
data_dir = "./data"

# Maximum collections per user
max_collections_per_user = 5

# Backup retention in days
backup_retention_days = 30

# Auto-backup on close
auto_backup = true

[media]
# Maximum file size in MB
max_file_size_mb = 100

# Allowed file extensions
allowed_extensions = [
  "jpg",
  "jpeg",
  "png",
  "gif",
  "svg",
  "mp3",
  "mp4",
  "ogg",
  "webm",
  "wav",
]

[limits]
# Maximum request size in MB
max_request_size_mb = 100

# Rate limiting (requests per minute per IP)
rate_limit_requests_per_minute = 60

# Maximum concurrent sessions per user
max_sessions_per_user = 3

[logging]
# Log level: trace, debug, info, warn, error
level = "info"

# Log file path
file = "logs/webapp.log"

# Log to stdout
stdout = true

[cors]
# Allow CORS (for API access from other origins)
enabled = false

# Allowed origins (if CORS enabled)
allowed_origins = ["http://localhost:3000"]
```

### Environment Variables

All configuration can be overridden with environment variables:

```bash
# Server
export ANKI_WEBAPP_HOST="0.0.0.0"
export ANKI_WEBAPP_PORT="8080"

# Auth
export ANKI_WEBAPP_JWT_SECRET="your-secret-here"
export ANKI_WEBAPP_SESSION_TIMEOUT_HOURS="24"

# Data
export ANKI_WEBAPP_DATA_DIR="/var/lib/anki-webapp"

# Logging
export RUST_LOG="anki_webapp=debug,tower_http=debug"

# Run
./anki-webapp
```

### HTTPS/TLS Setup

#### Option 1: Reverse Proxy (Recommended)

Use Nginx or Caddy as reverse proxy:

**Nginx** (`/etc/nginx/sites-available/anki-webapp`):

```nginx
server {
    listen 443 ssl http2;
    server_name anki.example.com;

    ssl_certificate /etc/letsencrypt/live/anki.example.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/anki.example.com/privkey.pem;

    location / {
        proxy_pass http://localhost:8080;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;
    }
}
```

**Caddy** (`Caddyfile`):

```
anki.example.com {
    reverse_proxy localhost:8080
}
```

## Usage

### First Time Setup

1. **Start the server** (see Installation above)

2. **Open browser** to `http://localhost:8080`

3. **Register account**:
   - Click "Register" on login page
   - Enter username, email, password
   - Click "Create Account"

4. **Create collection**:
   - After login, click "Create Collection"
   - Enter collection name
   - Click "Create"

5. **Start studying**!

### Daily Usage

#### Studying Cards

1. Navigate to **Decks**
2. Click **Study** on desired deck
3. Read question, think of answer
4. Press **Space** to reveal answer
5. Rate your recall:
   - **1** or **Again**: Didn't remember
   - **2** or **Hard**: Barely remembered
   - **3** or **Good**: Remembered correctly
   - **4** or **Easy**: Too easy

**Keyboard Shortcuts**:

- `Space`: Show answer / Next card
- `1/2/3/4`: Rate answer
- `E`: Edit current card
- `R`: Replay audio
- `Ctrl+Z`: Undo
- `F`: Flag card
- `@`: Suspend card
- `-`: Bury card

#### Adding Cards

1. Navigate to **Add** in menu
2. Select **Deck** from dropdown
3. Select **Note Type** (e.g., "Basic", "Cloze")
4. Fill in **Fields**:
   - Front: Question/Prompt
   - Back: Answer
5. Add **Tags** (optional)
6. Click **Add** or press `Ctrl+Enter`

**Media Upload**:

- Drag and drop images/audio into fields
- Or click 📎 icon to browse files
- Supported: Images (JPG, PNG, GIF), Audio (MP3, OGG), Video (MP4, WEBM)

#### Managing Decks

**Create Deck**:

1. Go to **Decks** page
2. Click **Create Deck**
3. Enter deck name (use `::` for subdecks, e.g., `Languages::French`)
4. Click **Create**

**Configure Deck**:

1. Click **Options** ⚙️ next to deck name
2. Adjust settings:
   - Daily new card limit
   - Daily review limit
   - Learning steps
   - FSRS parameters
3. Click **Save**

**Delete Deck**:

1. Click **⋮** menu next to deck name
2. Select **Delete**
3. Confirm (cards will also be deleted)

#### Browsing Cards

1. Navigate to **Browse** in menu
2. Enter search query or use filters:
   - `deck:"French"` - Cards in French deck
   - `is:new` - New cards
   - `is:due` - Cards due for review
   - `tag:vocabulary` - Cards with tag
   - `added:7` - Added in last 7 days
3. Select cards (Shift+Click for range, Ctrl+Click for multiple)
4. Use **Bulk Actions**:
   - Change deck
   - Add/remove tags
   - Suspend/unsuspend
   - Delete
   - Reschedule

### Advanced Features

#### Custom Study

Create temporary filtered deck:

1. Navigate to **Decks**
2. Click deck **⋮** menu
3. Select **Custom Study**
4. Choose study mode:
   - Increase today's limits
   - Study by card state or tags
   - Study ahead
   - Preview new cards
5. Click **Create**

#### Import/Export

**Import**:

1. Navigate to **Import** in menu
2. Select file type (APKG, CSV, TXT)
3. Upload file
4. Map fields (for CSV)
5. Click **Import**

**Export**:

1. Navigate to **Decks** or **Browse**
2. Select deck or cards to export
3. Click **Export**
4. Choose format:
   - APKG (with scheduling)
   - APKG (no scheduling)
   - CSV (plain text)
5. Download file

#### Statistics

1. Navigate to **Stats** in menu
2. Select time range (month, year, all time)
3. View graphs:
   - Forecast (upcoming reviews)
   - Reviews (past reviews)
   - Card Counts (by deck/type)
   - Intervals (card intervals distribution)
   - Hourly Breakdown (reviews by hour)
   - Answer Buttons (which buttons used)

## API Documentation

### Authentication

All API requests (except auth endpoints) require JWT token in header:

```bash
Authorization: Bearer <jwt_token>
```

### Get Token

```bash
# Login
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "user",
    "password": "password"
  }'

# Response:
{
  "success": true,
  "data": {
    "token": "eyJhbGc...",
    "user": {
      "id": "uuid",
      "username": "user",
      "email": "user@example.com"
    }
  }
}
```

### Example Requests

#### Get Decks

```bash
curl http://localhost:8080/api/v1/decks \
  -H "Authorization: Bearer <token>"
```

#### Get Next Card

```bash
curl http://localhost:8080/api/v1/scheduler/next \
  -H "Authorization: Bearer <token>"
```

#### Answer Card

```bash
curl -X POST http://localhost:8080/api/v1/scheduler/answer \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{
    "card_id": 123456789,
    "ease": 3,
    "time_taken_ms": 4500
  }'
```

#### Create Note

```bash
curl -X POST http://localhost:8080/api/v1/notes \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{
    "notetype_id": 1,
    "deck_id": 1,
    "fields": ["Front text", "Back text"],
    "tags": ["vocabulary"]
  }'
```

Full API documentation: [docs/webapp/api.yaml](docs/webapp/api.yaml) (OpenAPI spec)

Or visit: `http://localhost:8080/api/docs` (Swagger UI)

## Development

### Setting Up Development Environment

```bash
# Clone repository
git clone https://github.com/ankitects/anki.git
cd anki

# Backend
cd rslib/webapp
cargo build

# Frontend
cd ../../ts
npm install
npm run dev
```

### Running in Development Mode

**Terminal 1** - Backend (with auto-reload):

```bash
cd rslib/webapp
cargo watch -x run
```

**Terminal 2** - Frontend (with hot reload):

```bash
cd ts
npm run dev
```

**Terminal 3** - API requests:

```bash
# Backend API: http://localhost:8080
# Frontend dev server: http://localhost:5173
```

### Running Tests

```bash
# Backend tests
cd rslib/webapp
cargo test

# Frontend tests
cd ts
npm run test

# E2E tests
npm run test:e2e
```

### Code Quality

```bash
# Format code
./ninja format

# Lint
./ninja check

# Type check
cd ts && npm run check
```

## Deployment

### Production Checklist

- [ ] Change JWT secret in config
- [ ] Use strong passwords for accounts
- [ ] Enable HTTPS (via reverse proxy)
- [ ] Set up regular backups
- [ ] Configure firewall rules
- [ ] Set appropriate file permissions
- [ ] Enable rate limiting
- [ ] Monitor logs
- [ ] Set up log rotation
- [ ] Configure max upload sizes
- [ ] Test restore procedure

### Backup Strategy

**Automatic Backups**:

```toml
# config/server.toml
[collections]
auto_backup = true
backup_retention_days = 30
```

**Manual Backup**:

```bash
# Backup all user data
tar czf anki-backup-$(date +%Y%m%d).tar.gz data/

# Restore
tar xzf anki-backup-20240214.tar.gz
```

**Database Dump**:

```bash
# Backup user database
sqlite3 data/webapp.db .dump > webapp-backup.sql

# Restore
sqlite3 data/webapp.db < webapp-backup.sql
```

### Performance Tuning

**Database**:

```bash
# Optimize SQLite
sqlite3 data/webapp.db "VACUUM;"
sqlite3 data/webapp.db "ANALYZE;"
```

**Server**:

```toml
# config/server.toml
[server]
workers = 4 # Increase for more CPU cores

[limits]
max_request_size_mb = 50 # Reduce if low bandwidth
rate_limit_requests_per_minute = 120 # Adjust based on load
```

### Monitoring

**Logs**:

```bash
# Follow logs
tail -f logs/webapp.log

# Search for errors
grep ERROR logs/webapp.log

# Monitor in real-time
tail -f logs/webapp.log | grep -E "(ERROR|WARN)"
```

**Health Check**:

```bash
curl http://localhost:8080/api/v1/health
```

**Metrics** (optional):

```bash
# Prometheus metrics endpoint
curl http://localhost:8080/metrics
```

## Troubleshooting

### Server Won't Start

**Port already in use**:

```bash
# Check what's using port 8080
lsof -i :8080

# Kill process or change port in config
```

**Permission denied**:

```bash
# Check file permissions
ls -la /opt/anki-webapp

# Fix ownership
sudo chown -R anki:anki /opt/anki-webapp
```

### Can't Login

**Forgot password**:

```bash
# Reset password via CLI (future feature)
./anki-webapp reset-password username

# Or manually in database
sqlite3 data/webapp.db
UPDATE users SET password_hash = '<new_hash>' WHERE username = 'user';
```

**Invalid token**:

- Token expired (default 24 hours)
- JWT secret changed
- Login again to get new token

### Collection Won't Open

**Database locked**:

- Another instance accessing collection
- Kill other process or close from other session

**Corrupted database**:

```bash
# Check database integrity
sqlite3 collection.anki2 "PRAGMA integrity_check;"

# Restore from backup
cp backups/backup-2024-02-14/collection.anki2 .
```

### Poor Performance

**Slow queries**:

```bash
# Enable query logging
export RUST_LOG="sqlx=debug"

# Check slow queries in logs
grep "slow query" logs/webapp.log
```

**High memory usage**:

```bash
# Check memory
free -h

# Reduce workers if low memory
# Edit config/server.toml: workers = 2
```

**Slow media loading**:

- Check media file sizes
- Use appropriate formats (JPEG instead of PNG)
- Consider CDN for media files

### Common Errors

**"Collection not open"**:

- Open collection via UI or API first
- Check session is valid

**"Permission denied"**:

- Check file permissions on data directory
- User running server needs read/write access

**"Database disk image is malformed"**:

- Database corrupted
- Restore from backup
- Run `PRAGMA integrity_check;`

## FAQ

### Can I use my existing desktop collection?

Yes! Place your `.anki2` file in the appropriate user data directory:

```
data/users/{user_id}/collections/mycollection.anki2
```

### Can I use desktop and web app simultaneously?

No - both access the same SQLite database which doesn't support concurrent writers. Use one or the other.

### Does it sync with AnkiWeb?

Not yet - sync functionality is planned for a future release.

### Can multiple users access the same collection?

No - each user has isolated collections. Collections cannot be shared.

### How do I migrate from desktop to web app?

1. Export collection as `.apkg` from desktop
2. Import `.apkg` via web app
3. Or copy `.anki2` file to web app data directory

### Is mobile supported?

The web interface is responsive and works on mobile browsers. A dedicated mobile app is not yet available.

### Can I customize the interface?

Custom themes and CSS are not yet supported but planned for future releases.

### How secure is it?

- Passwords hashed with Argon2
- JWT tokens for authentication
- HTTPS recommended for production
- Rate limiting to prevent brute force
- Input validation and sanitization
- Follow security best practices for deployment

### What's the difference from AnkiDroid/iOS?

This is a web application accessed via browser, not a native mobile app. It can be self-hosted on your own server.

### Can I contribute?

Yes! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Support

- **Documentation**: [docs/webapp/](docs/webapp/)
- **Issues**: https://github.com/ankitects/anki/issues
- **Forum**: https://forums.ankiweb.net
- **Discord**: https://discord.gg/ankiforums

## License

Anki Web App is licensed under the same terms as Anki: AGPL-3.0-or-later

See [LICENSE](LICENSE) for details.
