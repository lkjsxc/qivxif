# Compose Pipeline

## Services

- `verify` runs `npm run build` (Vite), optional `npm run check` (svelte-check),
  Rust format check, clippy, workspace tests, and repository quality gates.
- `server-smoke` starts the server with an empty data directory.
- `api-test` runs API integration tests.
- `offline-e2e` runs browser offline checks when browser tests exist.

## Rule

Host commands are diagnostics. Completed slices pass through Docker Compose unless Docker is unavailable.
Verification containers mount the repository read-only and write generated browser assets to a scratch output path through `QIVXIF_WEB_DIST_DIR`.
Browser checks use Chromium in headless mode and keep API requests on the real server path.
