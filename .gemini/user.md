# User Preferences

## Build Environment

### Protobuf Compiler

- `protoc` is installed via Homebrew at `/opt/homebrew/bin/protoc` on macOS.
- On Linux, ensure `protoc` is in the `PATH`.
- **Environment**: When running cargo, ensure `PROTOC` is set if not in default path.

### n2 Build System

- Required for Anki build system.
- Path: `~/.cargo/bin/n2`
- **Command**: `export PATH="$HOME/.cargo/bin:$PATH"` before running `./check`.

## Git Configuration

### Repository Remotes

- **Origin (FORK)**: `git@github.com:kjwenger/anki.git`
- **Upstream (ORIGINAL)**: `git@github.com:ankitects/anki.git`
- **CRITICAL**: **NEVER PUSH TO UPSTREAM!** Always push to origin.

### User Info

- **Email**: kjwenger@yahoo.com
- **Name**: kjwenger
- **CONTRIBUTORS File**: Must include `kjwenger, at the domain yahoo.com` in alphabetical order (around line 52).

## Development Workflow

### Performance & Safety

- **Concise & Direct**: Senior engineer tone, minimal filler.
- **Surgical Changes**: Focus on the task, avoid unrelated refactors.
- **Check Frequently**: Run `./check` or specific sub-checks (clippy, svelte check) regularly.

### Reviewer UI Preferences

- Ensure keyboard shortcuts (`1-4`, `Space`, `Ctrl+Z`) are robust.
- Maintain responsive design for both mobile and desktop.
- Proper time tracking (`milliseconds_taken`) for cards.
