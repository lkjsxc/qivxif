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

Target repositories are defined in
[storage-repositories.md](storage-repositories.md): workspace layout, event
queue, graph projection, text snapshot, tab snapshot, tile layout, cache ledger,
inventory, and diagnostics.

Each repository exposes typed commands and typed query results. Transactions are
worker-owned envelopes, not component-owned storage calls.

The request and response contract is defined in
[storage-worker-protocol.md](storage-worker-protocol.md).

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
- `resource_entries`
- `resource_leases`
- `resource_journal`
- `resource_jobs`
- `media_assets`
- `media_chunks`
- `media_uploads`
- `profile_snapshots`
- `sync_cursors`

Schema names are durable `schema_contract` facts and are not inferred from UI
component names.

## Migration Rule

IndexedDB is not the durable product target and is not a hidden fallback. The
completed migration is tracked in [storage-migration.md](storage-migration.md),
[../../current-state.md](../../current-state.md), and
[../../product/doc-impl-audit.md](../../product/doc-impl-audit.md).

Dexie and active `indexedDB.open` calls are forbidden in product source.

## Diagnostics

Settings and Diagnostics render the record defined in
[storage-diagnostics.md](storage-diagnostics.md): mode, reason, quota, usage,
SQLite page count when available, inventory, queue counts, cache bytes, and the
last operation error.
