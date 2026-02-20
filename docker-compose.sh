#!/usr/bin/env bash
# Copyright: Ankitects Pty Ltd and contributors
# License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html
#
# Helper script to build the webapp and deploy it via Docker.

set -euo pipefail

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

BOLD='\033[1m'
GREEN='\033[0;32m'
CYAN='\033[0;36m'
RESET='\033[0m'

log() {
    local category=$1
    shift
    printf "[%s] %s  %-5s %s
" "$category" "$(date -u +"%Y-%m-%dT%H:%M:%S.%6NZ")" "INFO" "$*"
}

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$REPO_ROOT"

log "SYS" "=== Starting Anki Webapp Docker Deployment ==="

# ---------------------------------------------------------------------------
# 1. Build release artifacts
# ---------------------------------------------------------------------------

log "SYS" "Building release artifacts..."
./build-webapp.sh --release

# ---------------------------------------------------------------------------
# 2. Deploy via Docker Compose
# ---------------------------------------------------------------------------

log "SYS" "Deploying container..."
cd "$REPO_ROOT/docs/webapp"
docker compose up --detach --build

log "SYS" "=== Deployment Complete ==="
echo -e "
${GREEN}${BOLD}Anki Web App is running at http://localhost:5173${RESET}"
echo -e "View logs with: ${CYAN}docker compose logs -f${RESET}"
