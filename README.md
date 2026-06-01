# qivxif

qivxif is a Rust-first Web super app for graph-shaped knowledge work, offline editing, publishing, social feeds, and the `kjxlkj` graph composition workspace.

Durable truth belongs in [docs/](docs/). Implementation follows the owner docs and may be reshaped freely when the canon changes.

## Start Here

- [docs/README.md](docs/README.md): documentation map and reading order.
- [docs/vision/purpose.md](docs/vision/purpose.md): product purpose.
- [docs/decisions/accepted.md](docs/decisions/accepted.md): active architecture decisions.
- [docs/architecture/system/process-model.md](docs/architecture/system/process-model.md): process model.
- [docs/product/workspace/tiled-tabs.md](docs/product/workspace/tiled-tabs.md): workspace behavior.
- [docs/product/kjxlkj/graph-composition.md](docs/product/kjxlkj/graph-composition.md): `kjxlkj` behavior.
- [docs/operations/verification/compose-pipeline.md](docs/operations/verification/compose-pipeline.md): acceptance path.

## Current Direction

- Axum HTTP server.
- redb embedded storage.
- Typed KV plus typed graph data.
- Operation log and commit groups for history.
- Browser client with IndexedDB, service worker, and local-first sync.
- WebTransport as the preferred live sync lane.
- Tiled tabbed workspace as the first visible product surface.

## Development Rule

Make docs true first, then implementation true. Remove retired behavior instead of preserving old aliases.
