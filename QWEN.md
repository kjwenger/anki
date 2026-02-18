# Qwen Code Configuration

## Project Overview

Anki is a spaced repetition flashcard program with a multi-layered architecture. Main components:

- **Web frontend**: Svelte/TypeScript in `ts/`
- **PyQt GUI**: Embeds web components in `aqt/`
- **Python library**: Wraps Rust layer (`pylib/`, with Rust module in `pylib/rsbridge`)
- **Core Rust layer**: `rslib/`
- **Protobuf definitions**: `proto/` - used for cross-language communication

## Web App Extension (Current Focus)

The web app provides a headless, multi-user interface:

- **REST API server**: Rust/Axum in `rslib/webapp/`
- **Web frontend**: SvelteKit routes in `ts/routes/webapp/`
- **Documentation**: `SPECIFICATIONS.md`, `TASKS.md`, `PROJECT_LAYOUT.md`, `README.webapp.md`

## Building/Checking

```bash
./check  # Formats code and runs main build & checks
```

**Please do this as a final step before marking a task as completed.**

## Quick Iteration

During development, you can build/check subsections:

- **Rust**: `cargo check`
- **Python**: `./tools/dmypy` (if wheel-related: `./ninja wheels`)
- **TypeScript/Svelte**: `./ninja check:svelte`
- **Web App Backend**: `cd rslib/webapp && cargo check`
- **Web App Frontend**: `cd ts && npm run dev`

**Note**: Some changes (e.g., `.proto` files) need a full build with `./check` first.

## Build Tooling

`./check` and `./ninja` invoke the build system implemented in `build/`. It downloads required deps and invokes build steps.

## Translations

- **Location**: `ftl/` contains Fluent translation files
- **Auto-generation**: Scripts in `rslib/i18n` generate type-safe APIs for Rust, TypeScript, and Python
- **Changes**: Make changes to `ftl/core` or `ftl/qt` (prefer `core` unless Qt-specific)
- **Style**: Match existing style when adding new strings

## Protobuf and IPC

Build scripts use `.proto` files to define the Rust library's non-Rust API:

- `pylib/rsbridge` exposes the API
- `_backend.py` exposes `snake_case` methods for each protobuf RPC
- TypeScript gets `@generated/backend` module for Rust communication (over POST requests)
- **Web App REST API** also uses protobuf definitions, converting between protobuf and JSON

## Fixing Errors

When dealing with build errors or failing tests:

1. Invoke `check` or quick iteration commands regularly
2. This helps verify changes are correct
3. To locate other instances of a problem, run check again - **don't attempt to grep the codebase**

## Ignores

- `out/` contains auto-generated files - mostly ignore this folder
- May find it useful to view: `out/{pylib/anki,qt/_aqt,ts/lib/generated}` for cross-language communication or generated source code

## Launcher/Installer

- **Desktop launcher**: `qt/launcher` (separate code for each platform)
- **Web app deployment**: `deploy/` includes Docker configs and systemd service files

## Rust Dependencies

Prefer adding to the root workspace and using `dep.workspace = true` in individual Rust projects.

## Rust Utilities

`rslib/{process,io}` contain helpers for file and process operations:

- Better error messages/context and ergonomics
- Use them when possible
- Web app should also use these utilities for consistency

## Rust Error Handling

- **In rslib**: Use `error/mod.rs`'s `AnkiError/Result` and `snafu`
- **In other Rust modules**: Prefer `anyhow` + additional context where appropriate
- **Unwrapping**: Fine in build scripts/tests
- **Web app** (`rslib/webapp/`): Use `anyhow` with proper context, map to appropriate HTTP status codes in API layer

## Web App Styling

The webapp (`ts/routes/webapp/`) uses **Tailwind CSS v4** exclusively for all styling.

- Tailwind is loaded via the `@tailwindcss/vite` plugin (`ts/vite.config.ts`)
- The entry stylesheet is `ts/routes/webapp/app.css`, imported by `ts/routes/webapp/+layout.svelte`
- Dark mode is toggled by adding/removing the `dark` class on `<html>` (`@custom-variant dark` in `app.css`)
- Every webapp page must wrap its root element in `<div class="min-h-screen bg-gray-100 dark:bg-gray-900">` so the theme is applied consistently
- `app.css` uses `source(none)` on the utilities layer; source scanning is explicit via `@source` — keep those paths current when adding new directories

## Web App Development

### Testing

- **Backend**: Changes in `rslib/webapp/` should be tested with `cargo test`
- **Frontend**: Changes in `ts/routes/webapp/` should be tested with `npm run test`
- **API**: Follow REST conventions and update `docs/webapp/api.yaml`
- **Security**: Always validate inputs, use proper authentication
- **Performance**: Consider pagination, caching, and rate limiting

See `README.webapp.md` for detailed setup and usage instructions.

## Code Style

- **Rust**: Follow standard `rustfmt` settings (applied by `./check`)
- **TypeScript**: Follow `prettier` settings (applied by `./check`)
- **Svelte**: Use existing component patterns from `ts/lib/`
- **API**: Use consistent naming (`snake_case` for params, `camelCase` for JSON)

## Testing

- **Rust tests**: Place in same file or `tests/` subdirectory
- **TypeScript tests**: Place in `ts/tests/webapp/`
- **E2E tests**: Use Playwright in `ts/tests/e2e/`
- **Integration tests**: Test full API workflows in `rslib/webapp/tests/`

## Documentation

When adding new features:

- Update relevant `.md` files (`SPECIFICATIONS.md`, `README.webapp.md`)
- Add API documentation to `docs/webapp/api.yaml`
- Include code comments for complex logic
- Update `TASKS.md` to track completion

## Git Workflow

This is a fork at `git@github.com:kjwenger/anki.git`

- **Main upstream**: `origin` (`git@github.com:ankitects/anki.git`)
- **Fork**: `fork` (`git@github.com:kjwenger/anki.git`)
- **Push changes to fork**: `git push fork main`
- **Pull upstream updates**: `git pull --rebase origin main`

## Qwen Code Preferences

- **Be concise and direct** in responses
- **Provide working code examples** with context
- **Reference existing patterns** in the codebase
- **Run checks frequently** (`./check`)
- **Use parallel tool calls** for efficiency
- **Focus on minimal, surgical changes**
- **Validate changes** don't break existing functionality
- **Track progress** using `TASKS.md` and update completion status
- **Mark tasks complete** in `TASKS.md` when finished

## Current Project Status

**Phase 4.3 Complete** (Time Tracking Per Card) - 2026-02-18

All core API endpoints (Phase 2) and UI components (Phase 3) are implemented.
Current focus: Phase 4 - Desktop Parity Quick Wins

### Completed Phases

- ✅ **Phase 1**: Foundation (Authentication, Sessions, Configuration, Error Handling)
- ✅ **Phase 2**: Core API (All REST endpoints)
- ✅ **Phase 3**: UI Components (Auth, Collections, Decks, Reviewer, Editor, Browser, Stats, Settings)
- ✅ **Phase 4.1**: Critical Bug Fixes
- ✅ **Phase 4.2**: Interval Preview on Answer Buttons
- ✅ **Phase 4.3**: Time Tracking Per Card

### Next Tasks (Phase 4)

- **4.4**: Flag / Suspend / Bury During Review (Frontend work)
- **4.5**: Cloze Deletion Toolbar Helper
- **4.6**: Sticky Fields in Editor
- **4.7**: Duplicate Detection in Editor
- **4.8**: Deck Collapse / Expand State
- **4.9**: Overview Screen Before Study
- **4.10**: Audio Playback During Review
- **4.11**: Keyboard Shortcuts in Browse and Editor

See `TASKS.md` for detailed task breakdown and completion status.

## Key Files

- `SPECIFICATIONS.md` - Complete technical specifications
- `TASKS.md` - Implementation tasks with priorities and status (track completion here)
- `README.webapp.md` - Web app setup and usage guide
- `PROJECT_LAYOUT.md` - Project structure documentation
- `PHASE_*.md` - Phase completion reports
- `.qwen/user.md` - User-specific preferences and build notes
