# User Preferences

## Build Environment

### Protobuf Compiler

- `protoc` is installed via Homebrew at `/opt/homebrew/bin/protoc`. When running cargo commands, set `PROTOC=/opt/homebrew/bin/protoc` in the environment since the default PATH may not include `/opt/homebrew/bin`.

### n2 Build System

The Anki build system requires the `n2` build tool (Ninja alternative written in Rust).

**Installation:**
```bash
bash tools/install-n2
```

**Issue:** n2 is installed to `~/.cargo/bin/` but may not be in PATH by default.

**Solution:** Always run build commands with n2 in PATH:
```bash
export PATH="$HOME/.cargo/bin:$PATH"
./check
```

**Alternative:** Add to your shell profile (~/.bashrc, ~/.zshrc, etc.):
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

## Git Configuration

### Repository Remotes

**CRITICAL:** This is a fork of the upstream Anki repository.

- **Origin (MY FORK):** git@github.com:kjwenger/anki.git  
- **Upstream (ORIGINAL):** git@github.com:ankitects/anki.git

**⚠️ NEVER PUSH TO UPSTREAM!** Always push to my fork (origin).

**Setup remotes:**
```bash
# Verify current remotes
git remote -v

# If origin points to ankitects/anki, fix it:
git remote set-url origin git@github.com:kjwenger/anki.git

# Add upstream if not present:
git remote add upstream git@github.com:ankitects/anki.git

# Verify:
git remote -v
# Should show:
# origin    git@github.com:kjwenger/anki.git (fetch)
# origin    git@github.com:kjwenger/anki.git (push)
# upstream  git@github.com:ankitects/anki.git (fetch)
# upstream  git@github.com:ankitects/anki.git (push)
```

**Workflow:**
```bash
# Fetch latest from upstream
git fetch upstream

# Merge upstream changes to my main
git checkout main
git merge upstream/main

# Push to MY fork
git push origin main

# Work on features in branches
git checkout -b feature/my-feature
# ... make changes ...
git push origin feature/my-feature
```

### Git User Configuration

**Email:** kjwenger@yahoo.com

```bash
git config --global user.email "kjwenger@yahoo.com"
git config --global user.name "kjwenger"
```

### CONTRIBUTORS File

The build system (`./check`) validates that the git commit author is in the CONTRIBUTORS file.

**Format:** The CONTRIBUTORS file uses an anti-spam format:
```
kjwenger, at the domain yahoo.com
```

**Location:** After "kenden" in alphabetical order (around line 52)

**Validation:** The `tools/minilints` checks that the last commit author has previously modified the CONTRIBUTORS file. This means:
1. Add your email to CONTRIBUTORS in the correct alphabetical position
2. Commit that change with your configured git email
3. Future commits will pass validation

**How it works:**
- Script runs: `git log -1 --pretty=format:%ae` (gets last commit author)
- Script runs: `git log --pretty=format:%ae CONTRIBUTORS` (gets all authors who touched CONTRIBUTORS)
- If last author is in the list, check passes
- Otherwise, build fails with "Author NOT found in list"

## Build Process

### Running Checks

```bash
# Full build + tests + linting (required before committing)
export PATH="$HOME/.cargo/bin:$PATH"
./check

# Individual checks
cargo check                  # Rust compilation check
./ninja check:svelte        # Svelte/TypeScript check
./tools/dmypy              # Python type checking
cargo clippy --locked --tests -- -Dclippy::dbg_macro -Dwarnings  # Rust linting
```

### Common Build Issues

#### "n2 and ninja missing/failed"
**Cause:** n2 not in PATH  
**Fix:** `export PATH="$HOME/.cargo/bin:$PATH"`

#### "Author NOT found in list"
**Cause:** Git email not in CONTRIBUTORS file or not committed with that email  
**Fix:** 
1. Add email to CONTRIBUTORS (see above)
2. Commit with: `git commit -m "Add to CONTRIBUTORS"`
3. Ensure git config matches CONTRIBUTORS email

#### Unused imports / clippy warnings
**Fix:** Run `./check` - it will show specific issues to fix

## Development Notes

### Phase 2.5 - Cards API ✅ COMPLETE

**Status:** Implementation complete, build verified, documentation ready

**Completion Date:** 2026-02-15

**Files Created:**
- `rslib/webapp/src/routes/cards.rs` - Card route handlers (352 lines)
- `PHASE_2.5_COMPLETE.md` - Detailed completion report
- `CARDS_API_REFERENCE.md` - Quick reference guide

**Files Modified:**
- `rslib/webapp/src/routes/mod.rs` - Exports card routes
- `rslib/webapp/src/server/router.rs` - Router integration
- `rslib/webapp/src/openapi.rs` - OpenAPI documentation for Cards API (+290 lines)
- `rslib/webapp/src/error.rs` - Removed unused imports
- `rslib/webapp/src/routes/auth.rs` - Fixed duplicate import
- `rslib/webapp/src/db/mod.rs` - Fixed clippy auto-deref warning
- `CONTRIBUTORS` - Added kjwenger email entry
- `TASKS.md` - Marked phase complete

**Endpoints Implemented (9 total):**
- GET /api/v1/cards/{id} - Get card by ID
- PUT /api/v1/cards/{id} - Update card
- DELETE /api/v1/cards/{id} - Delete card
- POST /api/v1/cards/{id}/flag - Flag card (0-4)
- POST /api/v1/cards/{id}/suspend - Suspend card
- POST /api/v1/cards/{id}/unsuspend - Unsuspend/unbury card
- POST /api/v1/cards/{id}/bury - Bury card
- POST /api/v1/cards/batch - Get multiple cards by IDs
- POST /api/v1/cards/batch-update - Update multiple cards

**Build Status:**
- ✅ Compilation: Success (cargo build --release)
- ✅ Linting: Clean (cargo clippy)
- ✅ Integration: All routes registered with auth
- ✅ Documentation: Complete OpenAPI 3.0 spec
- ⏳ Full ./check: Pending CONTRIBUTORS git commit

**Key Learnings:**
- Anki uses protobuf types (`anki_proto::cards`, `anki_proto::scheduler`) for service APIs
- Service methods conflict with Collection inherent methods - use fully qualified syntax:
  ```rust
  <anki::collection::Collection as SchedulerService>::bury_or_suspend_cards(...)
  ```
- Service methods return `#[must_use]` types like `OpChanges` - use `let _ =` to explicitly ignore
- Cards API uses `CardsService` and `SchedulerService` traits from `anki::services`
- Protobuf request objects require all fields (e.g., `note_ids: vec![]` even if empty)
- `restore_buried_and_suspended_cards` is used for unsuspend (not a separate unsuspend method)

**Next Phase:** 2.6 Search API
