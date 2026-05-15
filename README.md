# qivxif

qivxif is a Rust-native tile workspace for writing, reading, browsing, and organizing local knowledge.

Durable truth belongs in `docs/`. Implementation follows the owner docs and may be reshaped freely when the canon changes.

## Start Here

- [docs/README.md](docs/README.md): documentation map and reading order.
- [docs/vision/purpose.md](docs/vision/purpose.md): product purpose.
- [docs/product/workspace/tile-layout.md](docs/product/workspace/tile-layout.md): visible workspace behavior.
- [docs/architecture/shell/event-loop.md](docs/architecture/shell/event-loop.md): desktop shell contract.
- [docs/operations/verification/compose-pipeline.md](docs/operations/verification/compose-pipeline.md): acceptance path.

## Current Direction

- Native desktop shell in Rust.
- Dockable tile workspace with stable pane identity.
- Text editor core with file-backed and scratch buffers.
- Markdown preview pane synchronized to editor state.
- Explorer pane for local project navigation.
- Browser pane with explicit policy and external fallback.
- Compose-backed checks for docs, build, tests, and smoke paths.

## Development Rule

Make docs true first, then implementation true. Remove retired behavior instead of preserving old aliases.
