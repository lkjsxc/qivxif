# Dependency Policy

## Defaults

- Rust-native dependencies.
- Boring stable crates for the initial slice.
- Add renderer and mobile dependencies only with owner docs and verification.

## Current Direction

- Tokio for async runtime.
- Quinn for QUIC.
- redb for hot state.
- serde JSON for simple initial protocol payloads.
