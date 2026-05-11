# Verification

## Rule

Docker Compose is the acceptance boundary.

```bash
./scripts/verify-compose.sh
```

## Static Gate

The `verify` service runs formatting, Clippy, tests, optimized build, docs
topology, and line limits.

## Live Probes

Live probes own readiness. Docker health checks are not used for acceptance.

## Stop Rule

No failing Compose gate may be ignored.

## Agent Evidence

Agents save verification output under `.sisyphus/evidence/` with task-scoped
filenames.

## Reproducibility

Use locked Cargo inputs and the pinned verify image. Do not rely on host-local
build output, redb files, or `tmp/` research files for acceptance.
