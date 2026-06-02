# Apps Overview

## Layout

- [qivxif-web](qivxif-web/README.md) is the only shipped browser package today.
- The Rust server binary lives at the repository root and serves `qivxif-web/dist/`.

## Build

```bash
npm --prefix apps/qivxif-web run build
```

Set `QIVXIF_WEB_DIST_DIR` when verify mounts the repository read-only.
