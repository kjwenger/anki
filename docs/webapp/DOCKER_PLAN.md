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

The deployment leverages a "Build on Host, Run in Container" strategy to avoid the complexity of building Anki's monorepo inside a container.

1. **Step 1**: Build the project artifacts locally.
2. **Step 2**: Copy artifacts into a lightweight `nginx:alpine` image.

## Prerequisites for Building

Before building the Docker image, you must build the release artifacts on your host:

```bash
# From project root
./build-webapp.sh --release
```

This ensures the `anki-webapp` binary and SvelteKit assets are ready in `target/release/` and `out/sveltekit/`.

## Deployment

```bash
cd docs/webapp
docker compose up -d --build
```

*Note: Always use `docker compose` (V2). The older hyphenated tool (`docker-compose`) may encounter compatibility issues with modern Docker versions.*

## Configuration

Environment variables in `docker-compose.yml`:

- `ANKI_WEBAPP_JWT_SECRET`: Secret for JWT token signing (change in production!)
- `RUST_LOG`: Log level (info, debug, warn, error)
- `ANKI_WEBAPP_DATA_DIR`: Where SQLite databases and media are stored (defaults to `/data` in container)

## Troubleshooting

### Container Name Conflicts
If you see an error about container name conflicts, remove the existing container first:
```bash
docker rm -f anki-webapp
```

### Port Conflicts
Ensure no native instances are running on ports 5173 or 8080:
```bash
./stop-webapp.sh
```

### Command Compatibility
Always use `docker compose` (without the hyphen). The older `docker-compose` tool may encounter `KeyError: 'ContainerConfig'` when interacting with modern Docker Engine versions.

## Data Persistence

User data (collections, media) is stored in the `anki-data` Docker volume mounted at `/data`.

## Notes

- Development should be done locally, not in containers
- This is for production deployment only
- The `./check` build system handles all code generation and dependency management
