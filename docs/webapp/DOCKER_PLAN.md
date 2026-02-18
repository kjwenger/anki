# Docker Deployment for Anki Web App

## Overview

Single-container production deployment combining:

- Rust API backend (port 8080)
- Svelte frontend served by Nginx (port 80)

## Architecture

```
┌─────────────────────────────────────────┐
│         Anki Web App Container          │
│                                         │
│  ┌──────────────┐    ┌──────────────┐   │
│  │   Nginx      │    │   Rust API   │   │
│  │   (Port 80)  │───▶│   (Port 8080)│   │
│  └──────────────┘    └──────────────┘   │
│         │                    │          │
└─────────┼────────────────────┼──────────┘
          │                    │
          ▼                    ▼
     Static Files          SQLite DB
                          (/data volume)
```

## Build Process

The Dockerfile uses multi-stage builds:

1. **Stage 1**: Build Rust API binary
2. **Stage 2**: Build Svelte frontend
3. **Stage 3**: Combine into nginx:alpine runtime

## Prerequisites for Building

Before building the Docker image, you must build the project locally:

```bash
# From project root
./check
```

This ensures all code generation and builds are complete.

## Deployment

```bash
cd docs/webapp
docker-compose up -d
```

## Configuration

Environment variables in `docker-compose.yml`:

- `JWT_SECRET`: Secret for JWT token signing (change in production!)
- `RUST_LOG`: Log level (info, debug, warn, error)
- `DATABASE_PATH`: Where SQLite databases are stored

## Data Persistence

User data (collections, media) is stored in the `anki-data` Docker volume mounted at `/data`.

## Notes

- Development should be done locally, not in containers
- This is for production deployment only
- The `./check` build system handles all code generation and dependency management
