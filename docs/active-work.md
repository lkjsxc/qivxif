# Active Work

## Current Objective

Harden the documented authoritative server slice before client or gameplay work.

## Work Lanes

- Documentation canon and repository rules are active.
- Rust workspace and Docker Compose verification are active.
- QUIC hello, join, chunk request, place, flush, restart, and persistence check
  are complete for the initial slice.
- Deterministic chunk generation and chunk-scoped edit overlay persistence are
  complete for the initial slice.

## Near-Term Exit Criteria

- Protocol ownership docs match the public wire catalog.
- Verification inputs are reproducible.
- Storage, protocol, and probe crates stay below line limits.
- Full `./scripts/verify-compose.sh` acceptance passes.
