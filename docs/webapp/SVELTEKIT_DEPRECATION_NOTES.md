# SvelteKit `config.kit.files` Deprecation Notes

## The Warnings

When building or type-checking the frontend, SvelteKit 2.x emits:

```
The `config.kit.files.lib` option is deprecated, and will be removed in a future version
The `config.kit.files.routes` option is deprecated, and will be removed in a future version
```

These come from `ts/svelte.config.js`:

```js
files: {
    lib: join(tsFolder, 'lib'),
    routes: join(tsFolder, 'routes'),
},
```

## Why They Cannot Be Eliminated

The entire `files.*` section of `kit` config has been marked deprecated in
SvelteKit 2.x. The intended long-term migration is toward a fully standardised
project layout where `lib` and `routes` live inside a single `src/` directory
(`src/lib/`, `src/routes/`).

**This project cannot use that standard layout.** The directories `ts/lib/` and
`ts/routes/` predate the webapp and contain the entire Anki desktop frontend
(deck options, graphs, image occlusion, etc.). Moving them to `ts/src/lib/` and
`ts/src/routes/` would require changes across the build system, import paths,
and every tool that references these directories.

### No Replacement Options Exist Yet

Despite the deprecation warnings, SvelteKit 2.50.2 provides **no alternative
configuration keys** for non-standard layouts:

- `files.lib` — used internally by the Vite plugin's `fs.allow` list and
  module resolution. Setting `kit.alias.$lib` alone is insufficient because the
  Vite plugin reads `kit.files.lib` directly for security allow-listing.
- `files.routes` — used by SvelteKit's routing manifest generator and type
  generator. There is no other key that accepts a custom routes path.

The `files.src` option (added in 2.28) is also deprecated and would not help
here: setting it to `tsFolder` would cause `app.html`, `hooks.client.js`, and
other files in `ts/src/` to no longer be found, since those depend on `files.src`
defaulting to `'src'`.

### Verified in SvelteKit Source

This was confirmed by reading the installed SvelteKit 2.50.2 source:

| File | Usage |
|------|-------|
| `src/core/config/options.js` | All `files.*` sub-keys wrapped in `deprecate()` |
| `src/exports/vite/index.js:254` | `kit.files.lib` added to Vite `fs.allow` set |
| `src/exports/vite/index.js:225` | `kit.files.lib` normalised for module resolution |
| `src/core/sync/write_tsconfig.js:67` | `kit.files.routes` and `kit.files.lib` in tsconfig includes |
| `src/core/sync/create_manifest_data/index.js:122` | `kit.files.routes` used for routing |

## Recommended Action

**Leave the config as-is.** The warnings are informational — they do not affect
the build or runtime behaviour. SvelteKit will need to provide a proper
migration path for non-standard project layouts before these options can be
removed. When that happens (likely a SvelteKit 3.x breaking change), the
appropriate fix will be to either:

1. Adopt the migration path SvelteKit provides, or
2. Restructure `ts/lib/` → `ts/src/lib/` and `ts/routes/` → `ts/src/routes/`
   as part of a planned refactor.

## Checked Against

- SvelteKit version: **2.50.2** (installed at `out/node_modules/@sveltejs/kit`)
- Config file: `ts/svelte.config.js`
- Date checked: 2026-02-18
