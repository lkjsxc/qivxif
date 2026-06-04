# Compose Pipeline

## Services

- `verify` runs the Vite browser build, optional Svelte check, Rust format check,
  clippy, workspace tests, and repository quality gates.
- `server-smoke` starts the optional sync and static-serving service with an empty data directory.
- `api-test` runs API integration tests for the optional service.
- `offline-e2e` runs browser offline checks when browser tests exist.

## Rule

Host commands are diagnostics. Completed slices pass through Docker Compose unless
Docker is unavailable. Use quiet progress for final gates.

Verification containers mount the repository read-only and write generated
browser assets to a scratch output path through `QIVXIF_WEB_DIST_DIR`.
Browser checks use Chromium in headless mode and keep API requests on the real
service path when sync is part of the slice.

## Final Gate

```text
docker compose --progress quiet -f docker-compose.yml build app verify offline-e2e
docker compose --progress quiet -f docker-compose.yml run --rm verify
docker compose --progress quiet -f docker-compose.yml run --rm offline-e2e
```

Passing quiet commands print one final success line or short stage lines. Captured
child output is printed only on failure.
