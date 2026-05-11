# Verification

## Rule

Docker Compose is the acceptance boundary.

## Static Gate

The `verify` service runs formatting, Clippy, tests, optimized build, docs
topology, and line limits.

## Live Probes

Live probes own readiness. Docker health checks are not used for acceptance.

## Stop Rule

No failing Compose gate may be ignored.
