# Web Layout

`apps/qivxif-web` owns browser source and emits static files served by `qivxif-server`.

## Source Tree

- `apps/qivxif-web/index.html`: app shell document.
- `apps/qivxif-web/src/main.ts`: browser entry point.
- `apps/qivxif-web/src/actors/`: app, workspace, sync, cache, editor, feed actors.
- `apps/qivxif-web/src/store/`: IndexedDB boundary.
- `apps/qivxif-web/src/ui/`: DOM rendering modules.
- `apps/qivxif-web/service-worker/`: service worker source.
- `apps/qivxif-web/public/`: manifest and static public assets.

## Build Output

- Build output directory is `apps/qivxif-web/dist`.
- The server reads `QIVXIF_STATIC_DIR`, defaulting to that path in development.
- The runtime image may copy `dist` to `/app/static`.
- `index.html`, compiled modules, service worker, manifest, and CSS are emitted together.

## Build Command

- `npm run build` builds browser assets when package metadata exists.
- `qivxifctl quality check-lines` covers TypeScript source under `apps/`.
- `scripts/verify-static.sh` runs the web build after the app shell exists.

## Rules

- Browser code may use TypeScript for Web APIs and UI boundaries.
- Durable reducers remain in Rust unless a documented WASM sharing step is added.
- Browser DTO names match [../../architecture/schema/README.md](../../architecture/schema/README.md).
