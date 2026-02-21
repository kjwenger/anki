#!/usr/bin/env bash
# Helper script to build the webapp and push it to a remote Docker registry.

set -euo pipefail

REGISTRY="registry.gertrun.synology.me"
IMAGE_NAME="anki/webapp"
TAG="latest"
REMOTE_IMAGE="$REGISTRY/$IMAGE_NAME:$TAG"

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

log "SYS" "=== Starting Anki Webapp Docker Push to $REGISTRY ==="

# 1. Build release artifacts
log "SYS" "Building release artifacts..."
./build-webapp.sh --release

# 2. Build Docker image
log "SYS" "Building Docker image..."
docker build -t "$IMAGE_NAME:$TAG" -f docs/webapp/Dockerfile .

# 3. Tag for remote registry
log "SYS" "Tagging image for remote registry: $REMOTE_IMAGE"
docker tag "$IMAGE_NAME:$TAG" "$REMOTE_IMAGE"

# 4. Push to remote registry
log "SYS" "Pushing to $REMOTE_IMAGE..."
docker push "$REMOTE_IMAGE"

log "SYS" "=== Push Complete ==="
echo -e "
${GREEN}${BOLD}Successfully pushed $REMOTE_IMAGE${RESET}"
