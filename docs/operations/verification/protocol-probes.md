# Protocol Probes

## Smoke Probe

- Connects over QUIC.
- Sends hello.
- Joins world.
- Requests one chunk.
- Sends ping.

## Persistence Probes

- `persist-place` places a block with a stable request identifier.
- `persist-place` forces a public flush before exiting.
- `persist-check` reconnects after restart and confirms the block.

## Rule

Probes use public protocol paths.
Probe messages, request identifiers, and response codes are owned by
[../../architecture/network/protocol-messages.md](../../architecture/network/protocol-messages.md).
