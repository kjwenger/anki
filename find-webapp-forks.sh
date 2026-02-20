#!/bin/bash
# Find forks of ankitects/anki that may have added a web app.
#
# Requires: gh (GitHub CLI), jq
#
# Usage:
#   ./find-webapp-forks.sh [--since YYYY-MM-DD] [--check-path PATH]
#
# Examples:
#   ./find-webapp-forks.sh
#   ./find-webapp-forks.sh --since 2024-01-01
#   ./find-webapp-forks.sh --since 2024-01-01 --check-path ts/routes/webapp

set -euo pipefail

# ---------------------------------------------------------------------------
# Defaults
# ---------------------------------------------------------------------------
SINCE="2024-01-01"
CHECK_PATH="ts/routes/webapp"
UPSTREAM="ankitects/anki"

# ---------------------------------------------------------------------------
# Argument parsing
# ---------------------------------------------------------------------------
while [[ $# -gt 0 ]]; do
    case "$1" in
        --since)       SINCE="$2";       shift 2 ;;
        --check-path)  CHECK_PATH="$2";  shift 2 ;;
        -h|--help)
            sed -n '2,12p' "$0" | sed 's/^# \?//'
            exit 0
            ;;
        *) echo "Unknown option: $1" >&2; exit 1 ;;
    esac
done

# ---------------------------------------------------------------------------
# Dependency checks
# ---------------------------------------------------------------------------
for cmd in gh jq; do
    if ! command -v "$cmd" &>/dev/null; then
        echo "ERROR: '$cmd' is required but not installed." >&2
        exit 1
    fi
done

# ---------------------------------------------------------------------------
# Logging helpers
# ---------------------------------------------------------------------------
log() {
    local level=$1; shift
    printf "[FORKS] %s  %-5s %s\n" "$(date -u +"%Y-%m-%dT%H:%M:%S")" "$level" "$*"
}

# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------
log "INFO" "Fetching forks of $UPSTREAM pushed since $SINCE ..."

mapfile -t FORKS < <(
    gh api "repos/$UPSTREAM/forks" \
        --paginate \
        --jq ".[] | select(.pushed_at > \"$SINCE\") | .full_name" \
    2>/dev/null
)

TOTAL=${#FORKS[@]}
log "INFO" "Found $TOTAL active forks — checking for '$CHECK_PATH' ..."
echo ""

FOUND=0
for repo in "${FORKS[@]}"; do
    if gh api "repos/$repo/contents/$CHECK_PATH" &>/dev/null; then
        log "INFO" "MATCH  https://github.com/$repo"
        FOUND=$((FOUND + 1))
    else
        log "DEBUG" "skip   $repo"
    fi
done

echo ""
log "INFO" "Done. $FOUND / $TOTAL active forks contain '$CHECK_PATH'."
