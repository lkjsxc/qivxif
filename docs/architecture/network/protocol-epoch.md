# Protocol Epoch

## Fields

- `build_epoch`: build identity from client and server.
- `protocol_epoch`: wire contract identity.
- `world_epoch`: persistent world identity.

## Rule

Do not use named product-line labels. Epoch fields describe contract gates
without implying a preserved old path.

## Current Protocol Epoch

- The active public `protocol_epoch` is `1`.
- A message-shape or enum-tag change requires an explicit epoch decision.
- Rejected hello mismatches return `ProtocolEpochMismatch`.

## Codec Link

Protocol message shape and accepted bytes are owned by
[protocol-codecs.md](protocol-codecs.md).
