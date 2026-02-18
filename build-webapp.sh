#!/usr/bin/env bash
# Copyright: Ankitects Pty Ltd and contributors
# License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html
#
# Rebuild the Anki web app from scratch:
#   - Rust backend  (rslib/webapp → target/.../anki-webapp)
#   - Svelte/TS frontend (ts/ → out/sveltekit/)
#
# Usage:
#   ./build-webapp.sh            # debug build
#   ./build-webapp.sh --release  # optimised release build

set -euo pipefail

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

BOLD='\033[1m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
CYAN='\033[0;36m'
RESET='\033[0m'

step()  { echo -e "\n${CYAN}${BOLD}==> $*${RESET}"; }
ok()    { echo -e "${GREEN}${BOLD}✓ $*${RESET}"; }
warn()  { echo -e "${YELLOW}${BOLD}⚠ $*${RESET}"; }
fail()  { echo -e "${RED}${BOLD}✗ $*${RESET}" >&2; exit 1; }

# ---------------------------------------------------------------------------
# Args
# ---------------------------------------------------------------------------

RELEASE=false
for arg in "$@"; do
    case "$arg" in
        --release) RELEASE=true ;;
        --help|-h)
            echo "Usage: $0 [--release]"
            echo "  (no flag)   debug build   – fast compile, slow binary"
            echo "  --release   release build – slower compile, optimised binary"
            exit 0
            ;;
        *) fail "Unknown argument: $arg" ;;
    esac
done

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$REPO_ROOT"

if $RELEASE; then
    CARGO_PROFILE="--release"
    PROFILE_DIR="release"
    BUILD_LABEL="release"
else
    CARGO_PROFILE=""
    PROFILE_DIR="debug"
    BUILD_LABEL="debug"
fi

echo -e "${BOLD}Anki web app rebuild  [${BUILD_LABEL}]${RESET}"
echo "Repository: $REPO_ROOT"

# ---------------------------------------------------------------------------
# 1. Rust backend
# ---------------------------------------------------------------------------

step "Building Rust backend (anki-webapp)"

if ! command -v cargo &>/dev/null; then
    fail "cargo not found – install Rust from https://rustup.rs"
fi

# shellcheck disable=SC2086
cargo build -p anki-webapp $CARGO_PROFILE

BACKEND_BIN="$REPO_ROOT/target/$PROFILE_DIR/anki-webapp"
[[ -x "$BACKEND_BIN" ]] || fail "Expected binary not found: $BACKEND_BIN"
ok "Backend binary: $BACKEND_BIN"

# ---------------------------------------------------------------------------
# 2. Svelte / TypeScript frontend
# ---------------------------------------------------------------------------

step "Building Svelte/TypeScript frontend"

if ! command -v yarn &>/dev/null; then
    fail "yarn not found – run: npm install -g yarn"
fi

# Install deps if node_modules is missing or package.json is newer
if [[ ! -d "$REPO_ROOT/node_modules" || "$REPO_ROOT/package.json" -nt "$REPO_ROOT/node_modules/.yarn-state.yml" ]]; then
    warn "node_modules missing or stale – running yarn install"
    yarn install
fi

yarn build

FRONTEND_OUT="$REPO_ROOT/out/sveltekit"
[[ -d "$FRONTEND_OUT" ]] || fail "Expected frontend output not found: $FRONTEND_OUT"
ok "Frontend output: $FRONTEND_OUT"

# ---------------------------------------------------------------------------
# Done
# ---------------------------------------------------------------------------

echo -e "\n${GREEN}${BOLD}Build complete!${RESET}"
echo
echo -e "  Backend  : ${BOLD}$BACKEND_BIN${RESET}"
echo -e "  Frontend : ${BOLD}$FRONTEND_OUT${RESET}"
echo
echo -e "Run the server with:"
echo -e "  ${BOLD}$BACKEND_BIN${RESET}"
echo -e "  (set ANKI_WEBAPP_HOST / ANKI_WEBAPP_PORT / ANKI_WEBAPP_JWT_SECRET as needed)"
