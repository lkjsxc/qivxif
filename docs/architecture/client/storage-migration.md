# Storage Migration

## Purpose

This doc owns the browser storage migration from the removed IndexedDB adapter
to worker-owned SQLite repositories.

## Target

- SQLite WASM runs in a dedicated browser worker.
- OPFS is the normal durable browser store.
- Memory SQLite is an explicit degraded store.
- Product code calls typed repositories.
- Product source does not call `indexedDB.open`.

## Current State

IndexedDB is not a product storage path and is not a hidden fallback. Dexie
remains forbidden.

The active browser source contains no `indexedDB.open` call. Local restore uses
the SQLite worker repository boundary through `src/lib/storage/current-store.ts`.

## Completed Steps

1. Defined typed repository interfaces and diagnostics DTOs.
2. Implemented the real SQLite worker, schema creation, and typed protocol.
3. Routed the controller and effects through the local store boundary.
4. Removed active imports from the IndexedDB adapter.
5. Deleted the IndexedDB adapter.
6. Strengthened repository gates so active IndexedDB storage cannot return.
7. Updated [../../current-state.md](../../current-state.md) and
   [../../product/doc-impl-audit.md](../../product/doc-impl-audit.md).

## Acceptance

- `state.storageStatus.mode` is `opfs`, `memory`, or `unavailable`.
- Settings and Diagnostics show worker diagnostics.
- Reload restores layout, tab drafts, scrolls, queue entries, graph projections,
  and text snapshots from SQLite when storage is durable.
- Offline queued mutations survive reload in durable mode.
- Docker Compose offline checks pass after the switch.
