# Browser Storage

## Purpose

Own typed local repositories backed by the SQLite worker.

## Allowed Imports

- Repository DTOs and diagnostics types from this directory.
- Worker protocol helpers from this directory.
- Browser worker construction in the storage client only.

## Forbidden Imports

- Svelte components.
- Optional server API clients.
- Raw SQL outside worker runtime and statement files.
- IndexedDB or Dexie.

## Owner Files

- `types.ts`: shared storage DTOs.
- `diagnostics.ts`: diagnostics defaults and helpers.
- `repositories.ts`: typed repository facade.
- `current-store.ts`: active store factory.
- `worker-protocol.ts`: request and response shapes.
- `sqlite-worker-client.ts`: only direct worker message client.
- `sqlite.worker.ts`: browser worker entry.
- `worker-runtime.ts`: SQLite connection and request handling.
- `sqlite-schema.ts`: schema creation text.
- `sqlite-statements.ts`: worker-owned SQL statements.

## Verification

Run web build, offline checks, and `qivxifctl quality check-browser-storage`.
