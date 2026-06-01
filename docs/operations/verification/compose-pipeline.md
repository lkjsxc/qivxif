# Compose Pipeline

## Services

- `verify` runs static gates.
- `server-smoke` starts the server with an empty data directory.
- `api-test` runs API integration tests.
- `offline-e2e` runs browser offline checks when browser tests exist.
- `webtransport-test` checks live sync when the endpoint exists.

## Rule

Host commands are diagnostics. Completed slices pass through Docker Compose unless Docker is unavailable.
