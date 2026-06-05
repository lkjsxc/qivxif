# Storage Worker Protocol

## Purpose

The storage worker protocol is the typed browser-host bridge for local SQLite.
It is a product contract, not a raw database escape hatch.

## Ownership

- The worker owns SQLite WASM startup, schema creation, transactions, and errors.
- The client owns request ids, bounded startup timeout, and typed repository
  method mapping.
- Product modules never post worker messages directly.

## Message Envelope

```typescript
type StorageWorkerRequest = {
  id: string;
  kind: StorageWorkerRequestKind;
  payload?: unknown;
};

type StorageWorkerResponse =
  | { id: string; ok: true; value: unknown }
  | { id: string; ok: false; error: StorageWorkerError };
```

`id` is unique per client request. Errors cross the worker boundary as data.
Exceptions do not escape as untyped worker failures.

## Request Kinds

```text
open
diagnostics.read
inventory.read
workspace.load
workspace.save_snapshot
queue.append_dirty_event
queue.list_all
queue.list_non_accepted
queue.mark_pending
queue.mark_accepted
queue.mark_rejected
graph.upsert_node_projection
graph.upsert_edge_projection
graph.list_nodes
graph.list_edges
graph.get_node
text.get_snapshot
text.save_snapshot
tabs.load_all
tabs.save_draft
tabs.save_scroll
tile.get_layout
tile.put_layout
tile.put_marker
cache.record_entry
cache.record_journal
```

## Transactions

The worker wraps these method groups in SQLite transactions:

- schema creation during `open`,
- workspace snapshot writes,
- event queue append with projection writes when supplied,
- pending, accepted, and rejected status transitions,
- tile layout snapshot plus marker updates,
- cache entry plus journal writes.

## Forbidden Payloads

- Raw SQL strings from product modules.
- Arbitrary table names from components.
- Unscoped worker messages outside `sqlite-worker-client.ts`.
- Hidden IndexedDB fallbacks.

## Startup

- `open` chooses OPFS-backed SQLite when available.
- If OPFS cannot be opened, `open` may choose real in-memory SQLite and must
  report degraded diagnostics.
- If SQLite cannot start, `open` returns `unavailable` diagnostics with a reason.
- Startup has a bounded timeout so the shell can show an honest degraded state.

## Error Shape

```typescript
type StorageWorkerError = {
  code: "startup_failed" | "not_open" | "transaction_failed" | "invalid_request";
  message: string;
  detail?: string;
};
```

The client records the last operation error in storage diagnostics.
