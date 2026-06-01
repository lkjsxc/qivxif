# Status Model

## Owner

The `qivxif-sync` crate owns the pure status model for pending local operations, push results, pull progress, and visible rejections.

Browser actors and server routes may store or expose the model, but they do not invent separate status names.

## Operation States

| State | Meaning | Durable Rule |
| --- | --- | --- |
| `dirty` | local operation is stored and awaiting upload | operation must already exist in durable local storage |
| `pending_validation` | upload is in flight | operation remains dirty-equivalent for cache protection |
| `rejected` | server returned structured rejection | operation stays visible until user or reducer resolves it |

Accepted operations leave the pending queue. The accepted server cursor updates `client_uploaded_through`.

## Model Fields

| Field | Type | Rule |
| --- | --- | --- |
| `pending` | ordered entries | sorted by actor sequence, then operation id |
| `client_uploaded_through` | optional `CursorId` | updated only by accepted push results |
| `client_applied_through` | optional `CursorId` | updated only after pull operations apply |
| `last_rejection` | optional rejection | mirrors the newest rejected operation |

## Reducer Rules

- Queueing an already queued operation is idempotent.
- Upload start changes `dirty` entries to `pending_validation`.
- A network failure returns in-flight operations to `dirty`.
- Accepted push results remove matching pending entries.
- Rejected push results keep matching entries with `rejected` state and error detail.
- Pull progress advances only after graph, text, or feed reducers apply the pulled operations.
- Timestamps never advance cursors.

## UI Mapping

- queued count is `dirty + pending_validation + rejected`.
- dirty count excludes rejected entries.
- rejected count is entries in `rejected` state.
- every rejection row includes operation id, code, and message.
- dirty and pending entries are protected from cache eviction.
