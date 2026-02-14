# Anki Web App

RESTful API server and web interface for Anki.

## Status

🚧 **Under Development** - Phase 1.1 Complete

Currently implemented:
- ✅ Basic server structure
- ✅ Configuration from environment variables
- ✅ Health check endpoint
- ✅ Hello world page

Coming next (see [TASKS.md](../../TASKS.md)):
- Authentication system
- Database for users
- Session management
- REST API endpoints
- Web UI

## Quick Start

```bash
# Run the server
cargo run

# Or with custom configuration
ANKI_WEBAPP_HOST=0.0.0.0 \
ANKI_WEBAPP_PORT=3000 \
RUST_LOG=debug \
cargo run
```

Then open http://localhost:8080 in your browser.

## Development

```bash
# Check code
cargo check

# Run tests
cargo test

# Run with auto-reload (requires cargo-watch)
cargo watch -x run

# Format code
cargo fmt
```

## Configuration

Set via environment variables:

- `ANKI_WEBAPP_HOST` - Bind address (default: 127.0.0.1)
- `ANKI_WEBAPP_PORT` - Port (default: 8080)
- `ANKI_WEBAPP_DATA_DIR` - Data directory (default: ./data)
- `ANKI_WEBAPP_JWT_SECRET` - JWT secret for auth (required in production)
- `RUST_LOG` - Log level (default: info)

## Documentation

See project root for complete documentation:

- [SPECIFICATIONS.md](../../SPECIFICATIONS.md) - Technical architecture
- [TASKS.md](../../TASKS.md) - Implementation roadmap
- [PROJECT_LAYOUT.md](../../PROJECT_LAYOUT.md) - Directory structure
- [README.webapp.md](../../README.webapp.md) - Usage guide
