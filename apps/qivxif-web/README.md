# qivxif Web App

This package owns the replaceable browser surface served by `qivxif-server`.
It uses small TypeScript modules and direct DOM rendering so LLM agents can
trace shell behavior without framework indirection.

## Contents

- [index.html](index.html): root app document.
- [package.json](package.json): browser build entry.
- [public/](public/): manifest and authored CSS.
- [scripts/](scripts/): static build copier.
- [service-worker/](service-worker/): service worker source.
- [src/](src/): browser actors, domain reducers, effects, and UI modules.

## Rules

- UI modules render state and emit actions.
- Actors own orchestration, persistence, and sync calls.
- Domain modules stay pure and do not touch DOM, storage, or network.
- Durable business rules must also exist in Rust reducers or server stores.
- Generated `dist/` files are build output, not source canon.
