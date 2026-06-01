# Status Model

## Owner

The `qivxif-sync` crate owns the pure status model for pending local events,
push results, pull progress, and visible rejections.

Browser actors and server routes may store or expose the model, but they do not invent separate status names.

## Event States

| State | Meaning | Durable Rule |
| --- | --- | --- |
| `dirty` | local event is stored and awaiting upload | event must already exist in durable local storage |
| `pending_validation` | upload is in flight | event remains dirty-equivalent for cache protection |
| `rejected` | server returned structured rejection | event stays visible until user or reducer resolves it |

Accepted events leave the pending queue. The accepted server cursor updates
`client_uploaded_through`.

## Model Fields

| Field | Type | Rule |
| --- | --- | --- |
| `pending` | ordered entries | sorted by actor sequence, then event id |
| `client_uploaded_through` | optional `CursorId` | updated only by accepted push results |
| `client_applied_through` | optional `CursorId` | updated only after pull events apply |
| `last_rejection` | optional rejection | mirrors the newest rejected event |

## Reducer Rules

- Queueing an already queued event is idempotent.
- Upload start changes `dirty` entries to `pending_validation`.
- A network failure returns in-flight events to `dirty`.
- Accepted push results remove matching pending entries.
- Rejected push results keep matching entries with `rejected` state and error detail.
- Pull progress advances only after graph, text, or feed reducers apply the pulled events.
- Timestamps never advance cursors.

## UI Mapping

- queued count is `dirty + pending_validation + rejected`.
- dirty count excludes rejected entries.
- rejected count is entries in `rejected` state.
- every rejection row includes event id, code, and message.
- dirty and pending entries are protected from cache eviction.
