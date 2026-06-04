# IndexedDB Cache Ledger Retirement

## Current Contract

IndexedDB is not the durable product storage target. Browser durable data belongs
to the SQLite worker described in [../client/sqlite-worker.md](../client/sqlite-worker.md).

## Allowed Uses

- Browser feature detection during migration.
- Historical tests that prove old data can be read before deletion.
- Service worker Cache API metadata only when routed through typed repositories.

## Forbidden Uses

- Svelte components opening IndexedDB.
- Product repositories choosing IndexedDB when SQLite is available.
- Dexie or other IndexedDB wrappers in active product source.
- Cache ledger repair or inventory sourced from IndexedDB after SQLite cutover.

## Cache Boundary

The Cache API may store app-shell assets and safe HTTP responses. Structured
product cache records, cache journal rows, inventory, protected bytes, and
prunable bytes belong to the SQLite worker repository boundary.
