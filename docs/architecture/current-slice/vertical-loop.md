# Vertical Loop

## Public Surface

The accepted public path is the same path used by the verification probes:

1. Connect to the server over QUIC.
2. Send `Hello` with non-empty build epoch and matching protocol epoch.
3. Send `JoinWorld`.
4. Send `ChunkRequest` for the target chunk.
5. Send `PlaceBlock` with a session-scoped request identifier.
6. Send `FlushPersistence` with a session-scoped request identifier.
7. Restart the server.
8. Send `ChunkRequest` for the same chunk.
9. Observe the placed block in the returned chunk cells.

## Acceptance

- The server never exposes storage internals through the protocol.
- The region actor is the only runtime owner of terrain mutation.
- The storage crate is the only redb table owner.
- The probe crate must use the public QUIC surface, not in-process calls.
