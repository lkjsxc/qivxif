# Storage Migration

## Purpose

This doc owns the active browser storage migration from the current IndexedDB
adapter to worker-owned SQLite repositories.

## Target

- SQLite WASM runs in a dedicated browser worker.
- OPFS is the normal durable browser store.
- Memory SQLite is an explicit degraded store.
- Product code calls typed repositories.
- Product source does not call `indexedDB.open`.

## Current Open Lane

IndexedDB is a migration input only. It is not the product storage target and it
is not a hidden fallback. Dexie remains forbidden.

The migration is complete only when active product source contains no
`indexedDB.open` call and local restore uses the SQLite worker repository
boundary.

## Steps

1. Define typed repository interfaces and diagnostics DTOs.
2. Implement the real SQLite worker, schema creation, and typed protocol.
3. Route the controller and effects through repositories.
4. Remove active imports from the IndexedDB adapter.
5. Delete the IndexedDB adapter.
6. Strengthen repository gates so active IndexedDB storage cannot return.
7. Update [../../current-state.md](../../current-state.md) and
   [../../product/doc-impl-audit.md](../../product/doc-impl-audit.md).

## Acceptance

- `state.storageStatus.mode` is `opfs`, `memory`, or `unavailable`.
- Settings and Diagnostics show worker diagnostics.
- Reload restores layout, tab drafts, scrolls, queue entries, graph projections,
  and text snapshots from SQLite when storage is durable.
- Offline queued mutations survive reload in durable mode.
- Docker Compose offline checks pass after the switch.
