# qivxif

qivxif is a browser-first Svelte workspace for graph-shaped knowledge work,
offline editing, Graph Map exploration, media handling, publishing, social feeds,
profiles, admin keys, resource orchestration, and tiled tabs.

Durable truth belongs in [docs/](docs/). Implementation follows the owner docs
and may be reshaped freely when the canon changes.

## Start Here

- [docs/README.md](docs/README.md): documentation map and reading order.
- [docs/current-state.md](docs/current-state.md): current evidence and open migration lanes.
- [docs/vision/purpose.md](docs/vision/purpose.md): product purpose.
- [docs/decisions/accepted.md](docs/decisions/accepted.md): active architecture decisions.
- [docs/architecture/client/app-shell.md](docs/architecture/client/app-shell.md): Svelte browser shell.
- [docs/architecture/client/wasm-boundary.md](docs/architecture/client/wasm-boundary.md): Svelte to Rust/WASM boundary.
- [docs/architecture/client/sqlite-worker.md](docs/architecture/client/sqlite-worker.md): worker-owned SQLite storage.
- [docs/product/tile-shell/tiled-tabs.md](docs/product/tile-shell/tiled-tabs.md): tile and tab behavior.
- [docs/product/graph-map/README.md](docs/product/graph-map/README.md): relationship map surface.
- [docs/product/resource-orchestration/README.md](docs/product/resource-orchestration/README.md): planner UX.
- [docs/product/doc-impl-audit.md](docs/product/doc-impl-audit.md): contract to implementation audit.
- [docs/operations/verification/compose-pipeline.md](docs/operations/verification/compose-pipeline.md): acceptance path.

## Current Direction

- Svelte owns every rendered product UI surface.
- Rust/WASM owns pure kernels, reducers, row codecs, and browser-host bridge logic.
- SQLite WASM in a worker owns browser durable state with OPFS as normal storage and explicit memory fallback.
- The shipped product has no Leptos UI shell, no Dexie path, no invented product data, and no hidden app backend requirement.
- Optional sync, serving, media, and admin code may exist only behind documented ports and must not leak into Svelte components.

## Development Rule

Make docs true first, then implementation true. Remove retired behavior instead of preserving old aliases.
