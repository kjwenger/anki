#!/usr/bin/env bash
# Copyright: Ankitects Pty Ltd and contributors
# License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html
#
# Run all automated tests for the Anki web app:
#   - Rust backend unit tests
#   - Rust backend integration tests
#   - TypeScript/Svelte unit tests (Vitest)
#   - SvelteKit type checking (svelte-check)

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

step()  { echo -e "
${CYAN}${BOLD}==> $*${RESET}"; }
ok()    { echo -e "${GREEN}${BOLD}✓ $*${RESET}"; }
warn()  { echo -e "${YELLOW}${BOLD}⚠ $*${RESET}"; }
fail()  { echo -e "${RED}${BOLD}✗ $*${RESET}" >&2; exit 1; }

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$REPO_ROOT"

echo -e "${BOLD}Anki web app test suite${RESET}"
echo "Repository: $REPO_ROOT"

# ---------------------------------------------------------------------------
# 1. Rust backend tests
# ---------------------------------------------------------------------------

step "Running Rust backend tests (unit & integration)"

if ! command -v cargo &>/dev/null; then
    fail "cargo not found – install Rust from https://rustup.rs"
fi

# Run all tests in the anki-webapp package
# This includes unittests in src/ and integration tests in tests/
cargo test -p anki-webapp

ok "Rust backend tests passed"

# ---------------------------------------------------------------------------
# 2. Svelte / TypeScript tests
# ---------------------------------------------------------------------------

step "Running Svelte/TypeScript unit tests (Vitest)"

if ! command -v yarn &>/dev/null; then
    fail "yarn not found – run: npm install -g yarn"
fi

# Run Vitest once
yarn vitest:once

ok "Svelte/TypeScript unit tests passed"

# ---------------------------------------------------------------------------
# 3. SvelteKit type checking
# ---------------------------------------------------------------------------

step "Running SvelteKit type check (svelte-check)"

# Run svelte-check once
yarn svelte-check:once

ok "SvelteKit type check passed"

# ---------------------------------------------------------------------------
# Done
# ---------------------------------------------------------------------------

echo -e "
${GREEN}${BOLD}All tests passed successfully!${RESET}"
