# Web Layout

`apps/qivxif-web` owns the SvelteKit browser client and emits static files served
by the optional sync service or any static host.

## Source Tree

- `apps/qivxif-web/src/routes/`: SvelteKit routes and root layout.
- `apps/qivxif-web/src/lib/app/`: controller, ports, workspace context.
- `apps/qivxif-web/src/lib/domain/`: pure workspace and tile reducers.
- `apps/qivxif-web/src/lib/effects/`: action table, sync, API adapters.
- `apps/qivxif-web/src/lib/storage/`: SQLite worker client and repositories.
- `apps/qivxif-web/src/lib/wasm/`: typed Rust/WASM service boundary.
- `apps/qivxif-web/src/lib/workspace/`: drag geometry helpers.
- `apps/qivxif-web/src/lib/components/`: workspace shell and product surfaces.
- `apps/qivxif-web/src/lib/styles/`: design tokens and layout CSS.
- `apps/qivxif-web/service-worker/`: service worker source.
- `apps/qivxif-web/static/`: manifest and icons.

## Build Output

- Vite writes to `apps/qivxif-web/dist/`.
- `QIVXIF_STATIC_DIR` points at `dist/` for service and smoke checks.
- `QIVXIF_WEB_DIST_DIR` redirects output for read-only verify mounts.

## Build Commands

```bash
npm --prefix apps/qivxif-web run build
npm --prefix apps/qivxif-web run check
```

- `qivxifctl quality check-lines` covers TypeScript and Svelte under `apps/`.
- `scripts/verify.sh` runs the web build inside Compose verify.

## Rules

- Durable reducers remain in Rust or WASM-backed services once shared.
- Browser DTO names match [../../architecture/schema/README.md](../../architecture/schema/README.md).
- Components never call raw storage or fetch directly.
