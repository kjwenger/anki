#!/usr/bin/env bash
# Copyright: Ankitects Pty Ltd and contributors
# License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html
#
# Gracefully stop the Anki webapp backend and frontend services.

set -euo pipefail

# ---------------------------------------------------------------------------
# Logging helpers
# ---------------------------------------------------------------------------

log() {
    local level=$1
    local category=$2
    shift 2
    printf "[%s] %s  %-5s %s
" "$category" "$(date -u +"%Y-%m-%dT%H:%M:%S.%6NZ")" "$level" "$*"
}

log "INFO" "SYS" "=== Stopping Anki Webapp ==="

# ---------------------------------------------------------------------------
# 1. Identify and stop API (Port 8080)
# ---------------------------------------------------------------------------

API_PORT=${ANKI_WEBAPP_PORT:-8080}
log "INFO" "API" "Checking for API process on port $API_PORT..."

# Find PID using lsof
API_PIDS=$(lsof -t -i :"$API_PORT" || true)

if [ -n "$API_PIDS" ]; then
    for pid in $API_PIDS; do
        log "INFO" "API" "Sending SIGTERM to API process (PID $pid)..."
        kill "$pid" || true
    done
else
    log "INFO" "API" "No API process found on port $API_PORT."
fi

# ---------------------------------------------------------------------------
# 2. Identify and stop Frontend (Port 5173)
# ---------------------------------------------------------------------------

FRONTEND_PORT=5173
log "INFO" "APP" "Checking for frontend process on port $FRONTEND_PORT..."

# Find PIDs using lsof (Node/Vite)
APP_PIDS=$(lsof -t -i :"$FRONTEND_PORT" || true)

if [ -n "$APP_PIDS" ]; then
    for pid in $APP_PIDS; do
        log "INFO" "APP" "Sending SIGTERM to frontend process (PID $pid)..."
        kill "$pid" || true
    done
else
    log "INFO" "APP" "No frontend process found on port $FRONTEND_PORT."
fi

# ---------------------------------------------------------------------------
# 3. Cleanup dangling Vite/esbuild services if any
# ---------------------------------------------------------------------------

log "INFO" "SYS" "Checking for dangling webapp-related processes..."
DANGLING_PIDS=$(ps aux | grep -E "anki-webapp|vite|svelte-kit" | grep -v grep | awk '{print $2}' || true)

if [ -n "$DANGLING_PIDS" ]; then
    for pid in $DANGLING_PIDS; do
        log "INFO" "SYS" "Cleaning up process (PID $pid)..."
        kill -9 "$pid" 2>/dev/null || true
    done
fi

log "INFO" "SYS" "Stop sequence complete."
