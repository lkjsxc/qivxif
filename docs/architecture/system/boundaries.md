# Boundaries

## Pure Domain

- ID parsing and formatting.
- Graph reducers.
- History projection.
- Text event grouping.
- Feed derivation.
- Cache planning.
- Tile layout reducers.
- Storage row codecs.

## Effectful Boundaries

- SQLite worker and OPFS access.
- WASM module loading.
- Axum handlers for optional service paths.
- redb transactions for optional service storage.
- WebTransport IO.
- Service worker cache writes.
- Password hashing.
- Time and randomness.

## UI Boundary

Svelte components render snapshots and emit commands. They do not own effects.
