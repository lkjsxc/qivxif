# qivxif

qivxif is a Rust-first Web super app for graph-shaped knowledge work, offline editing, publishing, social feeds, board composition, and a tiled tab shell.

Durable truth belongs in [docs/](docs/). Implementation follows the owner docs and may be reshaped freely when the canon changes.

## Start Here

- [docs/README.md](docs/README.md): documentation map and reading order.
- [docs/vision/purpose.md](docs/vision/purpose.md): product purpose.
- [docs/decisions/accepted.md](docs/decisions/accepted.md): active architecture decisions.
- [docs/architecture/system/process-model.md](docs/architecture/system/process-model.md): process model.
- [docs/product/setup/first-run.md](docs/product/setup/first-run.md): first-run owner setup.
- [docs/product/tile-shell/tiled-tabs.md](docs/product/tile-shell/tiled-tabs.md): tile and tab behavior.
- [docs/product/boards/composition.md](docs/product/boards/composition.md): board composition behavior.
- [docs/operations/verification/compose-pipeline.md](docs/operations/verification/compose-pipeline.md): acceptance path.

## Current Direction

- Axum HTTP server.
- redb embedded storage.
- Typed KV plus typed graph data.
- Operation log and commit groups for history.
- Browser client with IndexedDB, service worker, and local-first sync.
- WebTransport as the preferred live sync lane.
- Header plus tiled tab shell as the first visible product surface.

## Development Rule

Make docs true first, then implementation true. Remove retired behavior instead of preserving old aliases.
