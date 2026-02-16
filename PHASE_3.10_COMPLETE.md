# Phase 3.10 Complete: Graph Component Investigation

## Date
2026-02-16

## Summary
Investigated integrating Anki's existing Svelte graph components into the webapp statistics page.

## Investigation Findings

### Existing Graph Components
Found comprehensive graph library in `ts/routes/graphs/`:
- **13+ graph components**: Added, Buttons, Calendar, CardCounts, Difficulty, Ease, FutureDue, Hour, Intervals, Retrievability, Reviews, Stability, TrueRetention
- **Full Svelte implementation**: Ready-to-use components with styling
- **Rich visualization**: Uses D3.js-style rendering

### Current Architecture
1. **Desktop/Qt Mode**: 
   - Components fetch data via `@generated/backend`
   - Direct bridge to Rust backend via protobuf
   - Uses `graphs({ search, days })` RPC call

2. **Webapp Mode**:
   - REST API in `rslib/webapp/src/routes/stats.rs`
   - `get_graphs()` endpoint exists but marked "not implemented"
   - Issue: `GraphsResponse` protobuf doesn't implement `Serialize`

### Technical Challenge
The `GraphsResponse` protobuf structure is complex with:
- Nested maps and optional fields
- Multiple sub-messages (Added, Intervals, Eases, etc.)
- Would require manual serialization implementation

### Current Solution
- Basic statistics page with key metrics implemented
- `getTodayStats()` and `getCollectionStats()` REST endpoints working
- Clean, responsive UI with dark mode support

## Next Steps for Graph Integration
To fully integrate the rich graph components, we would need to:

1. **Option A - Full Protobuf Serialization**:
   - Implement `Serialize` for entire `GraphsResponse` structure
   - 200+ lines of manual conversion code
   - Complete graph data preservation

2. **Option B - Simplified Graph API**:
   - Create simplified REST endpoints for individual graphs
   - `/api/v1/stats/graphs/reviews`, `/api/v1/stats/graphs/intervals`, etc.
   - Easier to implement, less data overhead

3. **Option C - Hybrid Approach**:
   - Keep current statistics dashboard
   - Add selected high-value graphs (calendar, reviews, future due)
   - Implement only needed serialization

## Recommendation
Defer full graph integration to Phase 4 (Backend Integration & Advanced Features). The current statistics page provides essential metrics and is functional for MVP.

## Files Examined
- `ts/routes/graphs/*.svelte` - Graph components
- `ts/routes/graphs/WithGraphData.svelte` - Data fetching wrapper
- `rslib/src/stats/service.rs` - Rust stats service
- `rslib/webapp/src/routes/stats.rs` - REST API
- `proto/anki/stats.proto` - Protobuf definitions

## Status
✅ Investigation complete
✅ Architecture understood
⏸️ Full implementation deferred to Phase 4
