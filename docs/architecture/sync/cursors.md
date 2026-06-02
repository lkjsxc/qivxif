# Cursors

## Cursor Fields

- `server_seen_cursor`
- `client_uploaded_through`
- `client_applied_through`
- `per_text_doc_state`
- `per_feed_window_state`

## Server Cursor Contract

- A server cursor is a random `CursorId` generated at event acceptance.
- The cursor body does not encode time, event count, actor sequence, node ID, or
  event ID.
- The store maps each cursor to an internal acceptance sequence.
- Pull uses the internal sequence for ordering and returns the random cursor for
  resume.
- Replaying an accepted event returns the original cursor.
- Hidden events do not advance the cursor returned to a viewer.

## Rule

A single timestamp is never the only sync cursor, and cursor text is never a
correctness clock or a count leak.
