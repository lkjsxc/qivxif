# Networking References

## Findings

- QUIC matches mixed reliable and latest-wins game traffic.
- Quinn is the Rust transport choice.
- Reliable streams fit login, chunk bundles, inventory, abilities, and probes.
- Datagrams fit realtime input and ephemeral state after the initial slice.

## Current Status

The initial slice uses QUIC reliable streams.
