# GitHub Copilot CLI Configuration

## Project Overview

Anki is a spaced repetition flashcard program with a multi-layered architecture. Main components:

- Web frontend: Svelte/TypeScript in ts/
- PyQt GUI, which embeds the web components in aqt/
- Python library which wraps our Rust layer (pylib/, with Rust module in pylib/rsbridge)
- Core Rust layer in rslib/
- Protobuf definitions in proto/ that are used by the different layers to
  talk to each other.

## Web App Extension (New)

The web app provides a headless, multi-user interface:

- REST API server: Rust/Axum in rslib/webapp/
- Web frontend: SvelteKit routes in ts/routes/webapp/
- Documentation: SPECIFICATIONS.md, TASKS.md, PROJECT_LAYOUT.md, README.webapp.md

## Building/Checking

./check (check.bat) will format the code and run the main build & checks.
Please do this as a final step before marking a task as completed.

## Quick Iteration

During development, you can build/check subsections of our code:

- Rust: 'cargo check'
- Python: './tools/dmypy', and if wheel-related, './ninja wheels'
- TypeScript/Svelte: './ninja check:svelte'
- Web App Backend: 'cd rslib/webapp && cargo check'
- Web App Frontend: 'cd ts && npm run dev'

Be mindful that some changes (such as modifications to .proto files) may
need a full build with './check' first.

## Build Tooling

'./check' and './ninja' invoke our build system, which is implemented in build/.
It takes care of downloading required deps and invoking our build steps.

## Translations

ftl/ contains our Fluent translation files. We have scripts in rslib/i18n
to auto-generate an API for Rust, TypeScript and Python so that our code can
access the translations in a type-safe manner. Changes should be made to
ftl/core or ftl/qt. Except for features specific to our Qt interface, prefer
the core module. When adding new strings, confirm the appropriate ftl file
first, and try to match the existing style.

## Protobuf and IPC

Our build scripts use the .proto files to define our Rust library's
non-Rust API. pylib/rsbridge exposes that API, and _backend.py exposes
snake_case methods for each protobuf RPC that call into the API.
Similar tooling creates a @generated/backend TypeScript module for
communicating with the Rust backend (which happens over POST requests).

The web app REST API also uses these protobuf definitions, converting
between protobuf messages and JSON for HTTP requests.

## Fixing Errors

When dealing with build errors or failing tests, invoke 'check' or one
of the quick iteration commands regularly. This helps verify your changes
are correct. To locate other instances of a problem, run the check again -
don't attempt to grep the codebase.

## Ignores

The files in out/ are auto-generated. Mostly you should ignore that folder,
though you may sometimes find it useful to view out/{pylib/anki,qt/_aqt,ts/lib/generated}
when dealing with cross-language communication or our other generated sourcecode.

## Launcher/Installer

The code for our launcher is in qt/launcher, with separate code for each
platform.

The web app has its own deployment artifacts in deploy/ including Docker
configurations and systemd service files.

## Rust Dependencies

Prefer adding to the root workspace, and using dep.workspace = true in the
individual Rust project.

For the web app (rslib/webapp/), follow the same pattern and add shared
dependencies to the workspace Cargo.toml.

## Rust Utilities

rslib/{process,io} contain some helpers for file and process operations,
which provide better error messages/context and some ergonomics. Use them
when possible.

The web app should also use these utilities for consistency.

## Rust Error Handling

In rslib, use error/mod.rs's AnkiError/Result and snafu. In our other Rust
modules, prefer anyhow + additional context where appropriate. Unwrapping
in build scripts/tests is fine.

For the web app (rslib/webapp/), use anyhow for error handling with proper
context, and map to appropriate HTTP status codes in the API layer.

## Web App Development

When working on the web app:

- Backend: Changes in rslib/webapp/ should be tested with 'cargo test'
- Frontend: Changes in ts/routes/webapp/ should be tested with 'npm run test'
- API: Follow REST conventions and update docs/webapp/api.yaml
- Security: Always validate inputs, use proper authentication
- Performance: Consider pagination, caching, and rate limiting

See README.webapp.md for detailed setup and usage instructions.

## Code Style

- Rust: Follow standard rustfmt settings (applied by ./check)
- TypeScript: Follow prettier settings (applied by ./check)
- Svelte: Use existing component patterns from ts/lib/
- API: Use consistent naming (snake_case for params, camelCase for JSON)

## Testing

- Rust tests: Place in same file or tests/ subdirectory
- TypeScript tests: Place in ts/tests/webapp/
- E2E tests: Use Playwright in ts/tests/e2e/
- Integration tests: Test full API workflows in rslib/webapp/tests/

## Documentation

When adding new features:

- Update relevant .md files (SPECIFICATIONS.md, README.webapp.md)
- Add API documentation to docs/webapp/api.yaml
- Include code comments for complex logic
- Update TASKS.md to track completion

## Git Workflow

This is a fork at git@github.com:kjwenger/anki.git

- Main upstream: origin (git@github.com:ankitects/anki.git)
- Fork: fork (git@github.com:kjwenger/anki.git)
- Push changes to fork: 'git push fork main'
- Pull upstream updates: 'git pull --rebase origin main'

## Copilot CLI Preferences

- Be concise and direct in responses
- Provide working code examples
- Reference existing patterns in the codebase
- Run checks frequently (./check)
- Use parallel tool calls for efficiency
- Focus on minimal, surgical changes
- Validate changes don't break existing functionality
