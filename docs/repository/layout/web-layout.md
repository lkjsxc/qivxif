# Web Layout

`apps/qivxif-web` owns the SvelteKit browser client and emits static files served
by `qivxif-server`.

## Source Tree

- `apps/qivxif-web/src/routes/`: SvelteKit routes and root layout.
- `apps/qivxif-web/src/lib/app/`: controller, ports, bootstrap.
- `apps/qivxif-web/src/lib/domain/`: pure workspace and tile reducers.
- `apps/qivxif-web/src/lib/effects/`: IndexedDB, sync, API adapters.
- `apps/qivxif-web/src/lib/workspace/`: drag geometry ported from tiled shell reference.
- `apps/qivxif-web/src/lib/components/`: workspace shell and product surfaces.
- `apps/qivxif-web/src/lib/styles/`: design tokens and layout CSS.
- `apps/qivxif-web/service-worker/`: service worker source.
- `apps/qivxif-web/static/`: manifest and icons.

## Build Output

- Vite writes to `apps/qivxif-web/dist/`.
- `QIVXIF_STATIC_DIR` points at `dist/` for server and smoke services.
- `QIVXIF_WEB_DIST_DIR` redirects output for read-only verify mounts.

## Build Commands

```bash
npm --prefix apps/qivxif-web run build
npm --prefix apps/qivxif-web run check
```

- `qivxifctl quality check-lines` covers TypeScript and Svelte under `apps/`.
- `scripts/verify.sh` runs the web build inside Compose verify.

## Rules

- Durable reducers remain in Rust unless WASM sharing is documented.
- Browser DTO names match [../../architecture/schema/README.md](../../architecture/schema/README.md).
- Components never call IndexedDB or fetch directly.
