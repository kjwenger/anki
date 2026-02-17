#!/bin/bash
# Start the Anki webapp backend (API) and frontend (SvelteKit dev server)
#
# Backend API:  http://127.0.0.1:8080  (configurable via ANKI_WEBAPP_PORT)
# Frontend:     http://127.0.0.1:5173  (Vite dev server)

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

cleanup() {
    echo ""
    echo "Shutting down..."
    if [ -n "$API_PID" ] && kill -0 "$API_PID" 2>/dev/null; then
        kill "$API_PID"
        echo "Stopped API (PID $API_PID)"
    fi
    if [ -n "$FRONTEND_PID" ] && kill -0 "$FRONTEND_PID" 2>/dev/null; then
        kill "$FRONTEND_PID"
        echo "Stopped frontend (PID $FRONTEND_PID)"
    fi
    exit 0
}

trap cleanup INT TERM

echo "=== Starting Anki Webapp ==="
echo ""

# Start the backend API
echo "[API] Building and starting Rust backend..."
cd "$SCRIPT_DIR"
RUST_LOG="${RUST_LOG:-info}" cargo run --manifest-path rslib/webapp/Cargo.toml &
API_PID=$!
echo "[API] Started (PID $API_PID)"
echo ""

# Start the frontend dev server
echo "[Frontend] Starting Vite/SvelteKit dev server..."
cd "$SCRIPT_DIR"
yarn dev &
FRONTEND_PID=$!
echo "[Frontend] Started (PID $FRONTEND_PID)"
echo ""

echo "=== Both services starting ==="
echo "  API:      http://127.0.0.1:${ANKI_WEBAPP_PORT:-8080}"
echo "  Frontend: http://127.0.0.1:5173"
echo ""
echo "Press Ctrl+C to stop both services."
echo ""

# Wait for either process to exit
wait -n "$API_PID" "$FRONTEND_PID" 2>/dev/null || true
cleanup
