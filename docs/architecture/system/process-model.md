# Process Model

## Server Process

- Axum HTTP server listens on TCP.
- WebTransport server listens beside HTTP for live sync.
- Storage service owns redb access.
- Live session registry fans out post-commit notifications.

## Browser Process

- App shell renders the header and tile grid.
- IndexedDB adapter owns structured local persistence.
- Service worker owns app-shell cache and offline route.
- Sync actor pushes and pulls operations.

## CLI Process

`qivxifctl` owns admin bootstrap, store inspection, repair checks, feed rebuilds, and quality gates.
