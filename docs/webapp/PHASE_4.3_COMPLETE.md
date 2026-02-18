# Phase 4.3 Complete: Time Tracking Per Card

## Date
2026-02-18

## Summary
Implemented precise time tracking for each card review, recording the duration from card display to answer submission.

## Implementation Details

### Backend Changes (`rslib/webapp/src/routes/scheduler.rs`)
- Added `milliseconds_taken` field to `AnswerCardRequest` with a default value of 0.
- Updated `answer_card` handler to forward `milliseconds_taken` to the Anki core `CardAnswer` struct.
- This ensures the time spent is recorded in the `revlog` table for statistics.

### Frontend Changes (`ts/routes/webapp/review/+page.svelte`)
- Added `cardStartTime` variable to track the precise moment a card is displayed.
- Recorded `Date.now()` when `loadNextCard` successfully fetches a new card.
- Calculated the elapsed time in `answerCard` before sending the rating to the server.

### API & Documentation
- Updated `api.answerCard` in `ts/lib/webapp/api/client.ts` to accept and send `milliseconds_taken`.
- Updated OpenAPI documentation in `rslib/webapp/src/openapi.rs` to include the new field in `AnswerCardRequest`.

## Verification Results
- ✅ Card answer requests now include `milliseconds_taken`.
- ✅ Review statistics (Total Time, Average Time) correctly reflect the tracked duration.
- ✅ Build passes with `./check`.

## Status
✅ Phase 4.3 Complete
📋 Next Task: Phase 4.4 - Flag / Suspend / Bury During Review
