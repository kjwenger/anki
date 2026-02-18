# Competitive Analysis: AnkiWeb vs. Our Webapp

## What AnkiWeb Actually Provides

AnkiWeb (ankiweb.net) is primarily a **sync hub and thin browser client**, not a full web
application. Its browser experience is intentionally minimal — it mainly exists as a sync backend
for the desktop and mobile apps.

| Feature                             | AnkiWeb                   |
|-------------------------------------|---------------------------|
| Cloud sync (desktop ↔ mobile ↔ web) | ✅ Core feature            |
| Basic card review in browser        | ✅ Minimal UI              |
| Shared decks repository             | ✅ Large community library |
| Account / backup service            | ✅                         |
| Card editor in browser              | ❌ Very limited            |
| Statistics in browser               | ❌                         |
| Deck management in browser          | ❌                         |
| Open source                         | ❌ Proprietary             |
| Self-hostable                       | ❌                         |
| Multi-user / team support           | ❌                         |
| REST API                            | ❌                         |

## What Our Webapp Provides or Could Provide

| Feature                              | Status        |
|--------------------------------------|---------------|
| Card review with keyboard shortcuts  | ✅ Implemented |
| Deck browser & management            | ✅ Implemented |
| Card editor (plain text)             | ✅ Implemented |
| Card search (Anki query syntax)      | ✅ Implemented |
| Statistics & D3 graphs               | ✅ Implemented |
| Multi-user with JWT auth             | ✅ Implemented |
| REST API + Swagger UI (45 endpoints) | ✅ Implemented |
| Media upload (images, audio, video)  | ✅ Implemented |
| Tag management                       | ✅ Implemented |
| Self-hostable (Docker)               | ✅ Implemented |
| Open source (AGPL-3.0)               | ✅             |
| Rich text editor                     | ❌ Gap         |
| Interval preview on answer buttons   | ❌ Gap         |
| Deck options / FSRS parameters       | ❌ Gap         |
| `.apkg` import / export              | ❌ Gap         |
| AnkiWeb sync                         | ❌ Gap         |
| Shared decks repository              | ❌ Gap         |
| Note type management                 | ❌ Gap         |
| Cloze deletion toolbar               | ❌ Gap         |
| Audio recording                      | ❌ Gap         |

## Where Our Webapp Wins

1. **Richer browser experience** — AnkiWeb's browser UI is intentionally thin; ours is a full SPA
   with review, editing, browsing, and statistics.
2. **Self-hostable** — full data sovereignty, no dependency on Anki's servers.
3. **Open source** — forkable, auditable, and community-extensible under AGPL-3.0.
4. **REST API** — enables integrations, automation, and third-party clients.
5. **Multi-user** — suitable for team, school, or organization deployments.

## Where AnkiWeb Wins

1. **Sync ecosystem** — seamless sync with desktop and AnkiDroid/AnkiMobile is AnkiWeb's killer
   feature; our webapp is currently isolated.
2. **Shared decks** — massive community library of pre-made decks.
3. **Maturity & polish** — production-hardened, millions of users.
4. **Rich card rendering** — full LaTeX, audio replay, and image occlusion in review.
5. **FSRS / deck options** — fully configurable scheduler parameters per deck.

## AnkiWeb's Technology Stack

### Frontend

When fetching ankiweb.net, the page bootstraps a **SvelteKit** application — the same frontend
framework used in the open-source Anki `ts/` directory. This is no coincidence: Damien developed
the Svelte-based UI components as part of the open-source project and reuses them (or a private
fork of them) on AnkiWeb. The card reviewer, editor widgets, and statistics graphs in the
open-source repo were always designed to be embeddable in both the Qt desktop shell and a browser.

### Backend

AnkiWeb's server-side stack is **not publicly documented**. Given Damien's background and the
project's history, it is widely assumed to be Python-based, likely a lightweight custom framework
rather than Django or Flask. The sync protocol has been reverse-engineered by the community, which
led to open-source alternatives such as:

- **anki-sync-server** — a Rust-based sync server (now bundled with Anki 2.1.57+)
- **djankiserv** — a Django REST Framework implementation
- **anki-web-server** — a Flask-based minimal frontend

### Why It Is Not Based on This Repository

Damien has explained the decision directly on the Anki Forums:

> "To work full time on Anki, I need some way to pay the bills. Currently AnkiMobile is able to do
> that, but if one day its sales are no longer able to cover costs, I would need something to fall
> back on, and **AnkiWeb is a potential backup source of revenue**."

In other words, keeping the server-side code proprietary preserves the option to monetise AnkiWeb
(e.g. via a premium tier) without competitors being able to run identical infrastructure for free.
The community also noted that syncing tens of millions of daily reviews costs potentially tens of
thousands of dollars per month in server costs — costs that an open-source volunteer community
could not realistically absorb.

As a compromise, the built-in sync server was open-sourced and shipped with the desktop client,
so power users can self-host without depending on AnkiWeb at all.

### Relationship to Our Webapp

Our webapp shares the **same open-source Svelte/TypeScript frontend components** and the **same
Rust core library** as AnkiWeb's stack (at least on the client side). The key architectural
difference is the backend: we use an **Axum (Rust) REST API** rather than whatever proprietary
Python service AnkiWeb runs. This gives us a fully open, auditable, and self-hostable stack —
at the cost of not having AnkiWeb's sync infrastructure.

## Strategic Takeaway

Our webapp is actually **more capable as a browser application** than AnkiWeb — which is a low bar,
because AnkiWeb doesn't try to be one. The critical gap is the **sync ecosystem**: without `.apkg`
import/export and AnkiWeb sync support, users cannot move their existing collections in or out.
That is the highest-leverage gap to close.

## References

- [Syncing with AnkiWeb - Anki Manual](https://docs.ankiweb.net/syncing.html)
- [Anki - powerful, intelligent flashcards](https://apps.ankiweb.net/)
- [Anki (software) - Wikipedia](https://en.wikipedia.org/wiki/Anki_(software))
- [Open-sourcing AnkiWeb? - Anki Forums](https://forums.ankiweb.net/t/open-sourcing-ankiweb/4232)
- [Is the Ankiweb Frontend open source? - Anki Forums](https://forums.ankiweb.net/t/is-the-ankiweb-frontend-open-source/61411)
- [anki-sync-server (community) - GitHub](https://github.com/ankicommunity/ankicommunity-api-server)
- [djankiserv - Django-based sync server](https://github.com/wikidattica/djankiserv)
