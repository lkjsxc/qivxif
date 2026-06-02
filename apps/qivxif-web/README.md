# qivxif Web App

SvelteKit browser client served by `qivxif-server`.

## Contents

- [src/routes/](src/routes/): SvelteKit entry and layout.
- [src/lib/app/](src/lib/app/): controller, ports, workspace context.
- [src/lib/domain/](src/lib/domain/): pure tile and workspace reducers.
- [src/lib/effects/](src/lib/effects/): IndexedDB, sync, and API adapters.
- [src/lib/components/](src/lib/components/): workspace shell and product surfaces.
- [src/lib/styles/](src/lib/styles/): design tokens and layout CSS.
- [service-worker/](service-worker/): service worker source.
- [static/](static/): manifest and icons.

## Build

```bash
npm install
npm run build
```

Output is written to `dist/` for `QIVXIF_STATIC_DIR`.

## Rules

- Components emit actions; the controller owns state transitions.
- Domain modules stay pure with no DOM or network access.
- Source files stay at 200 lines or fewer.
