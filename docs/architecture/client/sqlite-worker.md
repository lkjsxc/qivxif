# SQLite Worker

## Ownership

A dedicated browser worker owns SQLite WASM. Product code calls typed
repositories and never opens SQLite, OPFS, IndexedDB, raw SQL, or worker messages
directly.

## Storage Modes

| Mode | Meaning | Product copy |
| --- | --- | --- |
| `opfs` | SQLite uses OPFS-backed durable storage | normal local storage |
| `memory` | OPFS is unavailable and SQLite runs in memory | degraded, reload may lose local changes |
| `unavailable` | Worker or SQLite cannot start | storage unavailable with reason |

The worker reports mode, reason, page count when available, estimated quota from
browser APIs, and last operation error.

## Repository Boundary

Target repositories:

- workspace layout repository,
- event queue repository,
- graph projection repository,
- text snapshot repository,
- tab snapshot repository,
- cache ledger repository,
- storage inventory repository.

Each repository exposes typed commands and typed query results. Transactions are
worker-owned envelopes, not component-owned storage calls.

## Tables

Browser SQLite tables mirror local product needs:

- `local_workspace`
- `events`
- `dirty_events`
- `accepted_events`
- `nodes`
- `edges`
- `text_snapshots`
- `tab_snapshots`
- `tile_layout`
- `feed_windows`
- `cache_entries`
- `cache_journal`
- `sync_cursors`

Schema names are durable `schema_contract` facts and are not inferred from UI
component names.

## Migration Rule

IndexedDB is not the durable product target. Existing IndexedDB code is an open
migration lane tracked in [../../current-state.md](../../current-state.md) and
[../../product/doc-impl-audit.md](../../product/doc-impl-audit.md).

Dexie is forbidden in active product source.

## Diagnostics

Settings and Diagnostics show:

- storage mode,
- unavailable or degraded reason,
- event queue counts,
- inventory by repository,
- protected and prunable cache bytes when the cache ledger exists,
- last storage operation error.
