# Buffer Core

Owner doc for text buffer internals.

## Model

- `BufferId` is a stable identity.
- The rope is the source of truth for text.
- Cursor and selection positions use buffer coordinates.
- Edit transactions update text, dirty state, and revision counters.

## Undo

- Each undo entry stores reversible edit spans.
- Coalescing policy lives in the buffer crate.
- New edits clear redo state.
- Tests must prove undo and redo round trips.

## Large Files

- Avoid full-file line string allocation.
- Cache line indexes separately.
- Cap undo history by bytes.
- Disable costly live features above configured thresholds.
