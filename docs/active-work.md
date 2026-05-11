# Active Work

## Current Objective

Improve documentation first, then implementation.

Current state: the authoritative server slice and headless protocol client are
implemented; there is no graphical client yet.

## Work Lanes

- Documentation canon and repository rules are active.
- Rust workspace and Docker Compose verification are active.
- Shared headless client core is active.
- QUIC hello, join, chunk, place, flush, restart, and persistence lanes are
  complete.
- `qivxif-client-cli` is part of Compose acceptance.
- Deterministic chunk generation and edit overlay persistence are complete.
- Protocol guard docs own the public session-phase rejection matrix.
- Malformed-wire docs own the public decode rejection contract.

## Near-Term Exit Criteria

- Owner docs match implemented behavior for the current slice.
- Protocol guard probes match the public wire catalog.
- Malformed-wire probes match the public decode rejection contract.
- Verification inputs are reproducible.
- Full `./scripts/verify-compose.sh` acceptance passes.
- Windows headless client artifact verification passes through Docker Compose.
