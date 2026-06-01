# Compose Runtime

## Services

- `qivxif`: server runtime using embedded redb storage.
- `verify`: static gates.
- `server-smoke`: empty data directory startup check.
- `api-test`: API integration checks.
- `offline-e2e`: browser offline checks when browser tests exist.
- `webtransport-test`: live sync checks when the endpoint exists.

## Rule

Core qivxif does not require PostgreSQL, Redis, S3, or hosted services.

## Runtime Paths

- Data directory: `/data`.
- Database file: `/data/qivxif.redb`.
- Static assets: `/app/static`.
- Bind address: `0.0.0.0:8080`.

## Acceptance

Compose services are the release boundary for completed slices. Host commands are diagnostics only.
