# Statistics/Graph Components: Reuse Analysis for Web App

## Current Architecture

The stats page lives in `ts/routes/graphs/` and follows this pipeline:

```
+page.svelte → GraphsPage.svelte → WithGraphData.svelte → Individual graphs
```

### Components

| Component                                                                                | Role                                                                                             |
|------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------|
| `+page.svelte`                                                                           | SvelteKit page entry; defines 14 graph components, sets initial state (`deck:current`, 365 days) |
| `WithGraphData.svelte`                                                                   | Centralized data orchestration; calls `graphs()` RPC, manages `GraphPreferences`                 |
| `GraphsPage.svelte`                                                                      | Responsive grid layout (3 → 2 → 1 columns); conditionally renders each graph                     |
| Individual graphs (e.g., `CardCounts.svelte`, `FutureDue.svelte`, `ReviewsGraph.svelte`) | Receive `sourceData` and `prefs`, transform data via TypeScript helpers, render with D3          |

### Rendering Technology

- **Svelte** for component structure and reactivity
- **D3.js** for imperative SVG chart drawing
- Reactive statements (`$:`) drive updates when `sourceData` or preferences change

### Data Flow

```
Rust Backend (rslib/stats)
    ↓
Protobuf RPC: graphs(GraphsRequest) → GraphsResponse
    ↓
@generated/backend (TypeScript generated code)
    ↓
WithGraphData.svelte (sourceData: GraphsResponse)
    ↓
Individual graphs → gatherData() → renderGraph()
```

**GraphsRequest**: `{ search: string, days: uint32 }`

**GraphsResponse** contains nested messages for:
- `CardCounts` (new, learn, relearn, young, mature, suspended, buried)
- `ReviewCountsAndTimes`
- `FutureDue` (future_due map, have_backlog, daily_load)
- `Added` (cards added per day)
- `Hours` (reviews by hour of day)
- `Today` (today's stats)
- `Eases`, `Difficulty`, `Intervals`, `Retrievability`, `Stability`
- `Buttons` (response button effectiveness)
- `TrueRetentionStats`

### Data Transformation Pattern

Each graph has a companion TypeScript module (e.g., `card-counts.ts`, `reviews.ts`) with pure functions:

```typescript
export function gatherData(data: GraphsResponse, ...options): GraphData
export function renderCards(svg: SVGElement, bounds, graphData): TableDatum[]
```

These transform modules are **pure TypeScript** with no Svelte or Qt dependencies.

### Qt Integration

The desktop app launches the stats page via `aqt/stats.py`:

```
Qt Dialog "NewDeckStats"
  → form.web.load_sveltekit_page("graphs")
  → QWebEngineView loads SvelteKit route
  → bridgeCommand("browserSearch:...") routes back to Qt
```

Key integration points:
- `QWebChannel` bridge for JavaScript → Python communication
- `browserSearch` command opens the desktop card browser
- `AuthInterceptor` injects auth headers for local API access
- `GraphPreferences` persisted via backend RPC to user profile

---

## Why the Existing Components Are Hard to Reuse

### 1. Monolithic Data Fetch

`WithGraphData.svelte` makes one large `graphs(search, days)` RPC that returns everything at once. This works for a local desktop app hitting a local backend, but for a web app over HTTP you'd want smaller, targeted API calls — not a single massive payload every time the user changes a filter.

### 2. D3 Direct DOM Manipulation

Graph components use D3 to imperatively mutate SVG elements (`d3.select(svg)...`). This fights against Svelte's declarative model and makes the components hard to compose, test, or adapt. Each graph component is essentially a self-contained D3 script wrapped in a Svelte shell.

### 3. Bridge Command Coupling

Graphs emit `browserSearch` events routed through Qt's `QWebChannel` bridge to open the desktop card browser (`aqt/stats.py`). In a web app there is no Qt browser to open, so this interaction model does not translate.

### 4. Preferences Tied to Desktop Backend

`GraphPreferences` are persisted via `setGraphPreferences()` RPC, which writes to the local Anki profile. A web app would need its own preference storage (localStorage, user account settings, etc.).

### 5. Tight Coupling to GraphsResponse Shape

Every graph component expects to destructure fields from a single protobuf response type. You cannot easily render just one graph without fetching the entire stats payload.

### 6. No Responsive/Mobile Consideration

The layout grid (`GraphsPage.svelte`) is designed for a desktop dialog window, not a full web app with navigation, routing, or mobile breakpoints.

---

## Reusability Assessment

| Layer                                                         | Reusability      | Notes                                                             |
|---------------------------------------------------------------|------------------|-------------------------------------------------------------------|
| Data transform modules (`card-counts.ts`, `reviews.ts`, etc.) | **High**         | Pure TypeScript functions, no framework dependencies              |
| Graph helper utilities (`graph-helpers.ts`)                   | **High**         | Enums, types, utility functions                                   |
| Protobuf data contracts (`stats.proto`)                       | **High**         | Can define API shape for any client                               |
| Individual graph Svelte components                            | **Low**          | D3 imperative rendering, tightly coupled to monolithic data shape |
| `WithGraphData.svelte` data orchestration                     | **Low**          | Monolithic fetch, desktop-oriented                                |
| `GraphsPage.svelte` layout                                    | **Low**          | Desktop dialog layout, Qt bridge event handling                   |
| Bridge/preference integration                                 | **Not reusable** | Qt-specific                                                       |

---

## Proposed Approaches

### Option A: Wrap and Adapt (Faster, Less Clean)

Keep the existing graph TypeScript transform modules — they are pure functions and genuinely reusable. Replace the rendering and data layers:

- Create a thin API adapter that fetches from the web backend and maps to the same `GraphsResponse` shape
- Replace D3 rendering with a declarative charting library (Chart.js, or Svelte-native like LayerCake/Pancake) for better Svelte integration
- Replace bridge commands with web-native actions (links, modals, etc.)

**Pros**: Faster to ship, reuses proven data logic
**Cons**: Still constrained by the monolithic data shape, adapter layer adds complexity

### Option B: Fresh Graph Components with Shared Data Contracts (Recommended)

- Define per-graph API endpoints (e.g., `/api/stats/card-counts`, `/api/stats/future-due`) instead of one monolithic call
- Build new Svelte components using a declarative chart library
- Import the existing `gatherData()` transform functions where they work
- Store preferences in localStorage or the webapp's user settings
- Design for mobile-first responsive layout

**Pros**: Clean architecture for web context, better performance (fetch only what you need), mobile-ready
**Cons**: More upfront work, parallel maintenance of graph rendering code

### Recommendation

**Option B.** The reusable parts are roughly 30% of the existing code — the pure data transforms. The rendering (D3 imperative SVG), data fetching (monolithic protobuf), and interaction model (Qt bridge) all need replacement. Starting fresh with the right abstractions for a web context while reusing the data transformation logic will produce a better result than shimming desktop components into a web app.
