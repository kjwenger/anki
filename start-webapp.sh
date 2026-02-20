#!/bin/bash
# Start the Anki webapp backend (API) and frontend (SvelteKit dev server)
#
# Backend API:  http://127.0.0.1:8080  (configurable via ANKI_WEBAPP_PORT)
# Frontend:     http://127.0.0.1:5173  (Vite dev server)

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# ---------------------------------------------------------------------------
# Logging helpers
# ---------------------------------------------------------------------------

# Helper to print unified logs for the script itself
# Format: [CATEGORY] TIMESTAMP  LEVEL  message
# Usage: log <LEVEL> <CATEGORY> <MESSAGE>
log() {
    local level=$1
    local category=$2
    shift 2
    printf "[%s] %s  %-5s %s\n" "$category" "$(date -u +"%Y-%m-%dT%H:%M:%S.%6NZ")" "$level" "$*"
}

# Helper to prefix piped subprocess output with the category tag.
# The subprocess (e.g. Rust tracing) already emits "TIMESTAMP  LEVEL  message",
# so we just prepend "[CATEGORY] " — no re-wrapping, no double timestamps.
# Usage: command | prefix_logs <CATEGORY>
prefix_logs() {
    local category=$1
    while IFS= read -r line; do
        printf "[%s] %s\n" "$category" "$line"
    done
}

cleanup() {
    echo ""
    log "INFO" "SYS" "Shutting down..."
    if [ -n "$API_PID" ] && kill -0 "$API_PID" 2>/dev/null; then
        kill "$API_PID"
        log "INFO" "API" "Stopped (PID $API_PID)"
    fi
    if [ -n "$FRONTEND_PID" ] && kill -0 "$FRONTEND_PID" 2>/dev/null; then
        kill "$FRONTEND_PID"
        log "INFO" "APP" "Stopped (PID $FRONTEND_PID)"
    fi
    exit 0
}

trap cleanup INT TERM

log "INFO" "SYS" "=== Starting Anki Webapp ==="

# Start the backend API
log "INFO" "API" "Building and starting Rust backend..."
cd "$SCRIPT_DIR"
# Pipe both stdout and stderr through the prefixer
RUST_LOG="${RUST_LOG:-info}" cargo run --manifest-path rslib/webapp/Cargo.toml 2>&1 | prefix_logs "API" &
API_PID=$!
log "INFO" "API" "Process started (PID $API_PID)"

# Start the frontend dev server
log "INFO" "APP" "Starting Vite/SvelteKit dev server..."
cd "$SCRIPT_DIR"
# Force color for yarn/vite output and pipe through prefixer
FORCE_COLOR=1 yarn dev 2>&1 | prefix_logs "APP" &
FRONTEND_PID=$!
log "INFO" "APP" "Process started (PID $FRONTEND_PID)"

log "INFO" "SYS" "=== Both services starting ==="
log "INFO" "SYS" "API:      http://127.0.0.1:${ANKI_WEBAPP_PORT:-8080}"
log "INFO" "SYS" "Frontend: http://127.0.0.1:5173"
log "INFO" "SYS" "Press Ctrl+C to stop both services."

# Wait for either process to exit
wait -n "$API_PID" "$FRONTEND_PID" 2>/dev/null || true
cleanup
