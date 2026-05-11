# Networking References

## Scope

This file summarizes transport findings. Protocol ownership remains under the
architecture network docs.

## Findings

- QUIC matches mixed reliable and latest-wins game traffic.
- Quinn is the Rust transport choice.
- Reliable streams fit login, chunk bundles, inventory, abilities, and probes.
- Datagrams fit realtime input and ephemeral state after the initial slice.
- Public behavior should be proven through QUIC and `postcard`, not in-process
  shortcuts.
- `postcard` is compact and schema-bound; it is not self-describing.

## Current Status

The initial slice uses QUIC reliable streams.
