# Gemini CLI - Anki Web App

## Project Overview

Anki is a spaced repetition flashcard program with a multi-layered architecture. Main components:

- **Web Frontend**: Svelte/TypeScript in `ts/`
- **PyQt GUI**: Embeds web components in `aqt/`
- **Python Library**: Wraps Rust layer (`pylib/`, with Rust module in `pylib/rsbridge`)
- **Core Rust Layer**: `rslib/`
- **Protobuf Definitions**: `proto/` for inter-layer communication
- **Web App Extension**:
  - **REST API Server**: Rust/Axum in `rslib/webapp/`
  - **Web Frontend**: SvelteKit routes in `ts/routes/webapp/`

## Building/Checking

- **Full Build & Checks**: `./check` (or `check.bat`)
  - Formats code, runs main build, tests, and lints.
  - **Mandatory** before completion of any task.
- **n2 Build System**: Required (Ninja alternative in Rust).
  - Install: `bash tools/install-n2`
  - Ensure in PATH: `export PATH="$HOME/.cargo/bin:$PATH"`
- **Protobuf Compiler**:
  - `protoc` path: `/opt/homebrew/bin/protoc` (on macOS) or ensure `protoc` is in `PATH`.

## Quick Iteration

- **Rust**: `cargo check`
- **Python**: `./tools/dmypy`, and if wheel-related, `./ninja wheels`
- **TypeScript/Svelte**: `./ninja check:svelte`
- **Web App Backend**: `cd rslib/webapp && cargo check`
- **Web App Frontend**: `cd ts && npm run dev`

## Styling Conventions

- **Webapp (`ts/routes/webapp/`)**: **Tailwind CSS v4** exclusively.
- **Entry Stylesheet**: `ts/routes/webapp/app.css`
- **Root Wrapper**: `<div class="min-h-screen bg-gray-100 dark:bg-gray-900">`
- **Dark Mode**: Uses `dark` class on `<html>`.

## Technical Integrity

- **Protobuf Conversion**: Web app REST API converts between Protobuf and JSON.
- **Translations**: Use Fluent (`ftl/`). Changes in `ftl/core` (preferred) or `ftl/qt`.
- **Rust Error Handling**:
  - `rslib`: `AnkiError`/`Result` + `snafu`.
  - Web App: `anyhow` + context, mapped to HTTP status codes.
- **Utilities**: Use `rslib/{process,io}` for file and process operations.

## Git Workflow

- **Origin (Fork)**: `git@github.com:kjwenger/anki.git`
- **Upstream (Original)**: `git@github.com:ankitects/anki.git`
- **Push**: Always push to **origin**.
- **CONTRIBUTORS**: Mandatory update for new authors; `./check` validates commit author against this file.

## Instructions & Mandates

- **Explain Before Acting**: Provide a concise one-sentence explanation of intent before tool calls.
- **Verification is Mandatory**: Run tests and `./check` before declaring a task complete.
- **Tailwind v4**: Adhere strictly to Tailwind CSS v4 for all `ts/routes/webapp/` components.
- **API Documentation**: Update `rslib/webapp/src/openapi.rs` and `docs/webapp/api.yaml` for API changes.
- **Phase Completion**: Document completion in a `PHASE_X.Y_COMPLETE.md` file and update `TASKS.md` and `PROJECT_STATUS.md`.

See `.gemini/user.md` for user-specific preferences and environment configuration.
