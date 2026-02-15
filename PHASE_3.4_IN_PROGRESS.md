Phase 3.4 Study Interface - Status: In Progress

The scheduler API routes have been created but need additional work to properly integrate
with Anki's internal rendering APIs. The challenge is that Anki's card rendering requires
multiple internal components (Note, Notetype, CardTemplate) that are not easily accessible
from the current protobuf-based service API.

Next steps:
1. Create a simplified notetype service wrapper that can render cards with just a card ID
2. OR use a different approach - return raw card data and render on the client side
3. Implement the Svelte UI components for the study interface

Files created (partial):
- rslib/webapp/src/routes/scheduler.rs - Scheduler REST endpoints (needs completion)
- rslib/webapp/src/routes/mod.rs - Added scheduler module
- rslib/webapp/src/server/router.rs - Added scheduler routes

This task requires deeper understanding of Anki's card rendering pipeline.

