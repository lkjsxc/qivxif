# Process Model

## Browser Process

- Svelte app shell renders the header and tile grid.
- Controller owns workspace state and dispatch.
- SQLite worker owns browser durable storage.
- WASM services own deterministic kernels and codecs.
- Service worker owns app-shell cache and offline route.
- Sync actor pushes and pulls events only through documented ports.

## SQLite Worker

- Starts before durable local reads.
- Chooses OPFS when available.
- Falls back to memory only with an explicit degraded diagnostic.
- Exposes typed repository messages, not raw SQL messages.
- Reports mode, reason, inventory, and last operation error.

## Optional Sync Service

- Axum may serve HTTP API, public routes, auth routes, and static assets.
- redb may back accepted event storage for that service.
- The browser remains usable without the service after assets and local storage open.
- Product UI treats service failures as sync diagnostics, not local acceptance.

## CLI Process

`qivxifctl` owns admin bootstrap, event and store inspection, repair checks,
feed rebuilds, and quality gates for the optional sync service and repository.
