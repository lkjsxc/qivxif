# Protocol Probes

## Smoke Probe

- Connects over QUIC.
- Sends hello.
- Joins world.
- Requests one chunk.
- Sends ping.

## Persistence Probes

- `persist-place` places a block.
- `persist-check` reconnects after restart and confirms the block.

## Rule

Probes use public protocol paths.
