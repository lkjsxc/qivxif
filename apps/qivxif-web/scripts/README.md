# Web Scripts

This directory owns package-level utility scripts.

## Contents

- [build.mjs](build.mjs): copies TypeScript modules, public assets, the app
  document, and the service worker into the configured build output directory.

## Rules

- The build remains intentionally small until docs choose a richer toolchain.
- `QIVXIF_WEB_DIST_DIR` may redirect output for read-only Docker verification.
- Build scripts must not encode product behavior.
