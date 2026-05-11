# Dependency Policy

## Defaults

- Rust-native dependencies.
- Boring stable crates for the initial slice.
- Add renderer and mobile dependencies only with owner docs and verification.

## Current Direction

- Tokio for async runtime.
- Quinn for QUIC.
- redb for hot state.
- `postcard` for compact schema-bound protocol payloads.
- serde JSON only for diagnostics or tool output that is not protocol traffic.
