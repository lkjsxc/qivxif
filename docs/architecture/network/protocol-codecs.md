# Protocol Codecs

## Canon

Use `postcard` for compact protocol messages on reliable streams and datagram
lanes.

## Epoch Ownership

- `protocol_epoch` owns message shape, enum tags, and field meaning.
- `build_epoch` owns client and server build identity.
- `world_epoch` owns persistent world identity.
- A codec change that changes accepted bytes requires an epoch decision.

## Rules

- Protocol payloads are schema-bound and non-self-describing.
- Every protocol message has one owner doc and one owner crate.
- Mutating requests carry stable request identifiers.
- Authoritative acknowledgements echo the request identifier.
- Error codes are typed protocol values; message text is diagnostic only.
- Hello responses include server capabilities for the active epoch.
- Datagram payloads stay small and latest-wins.
- Reliable keyframes repair state that can be lost on datagram lanes.
- `rkyv` is not used for client-trusted wire messages.

## Archive Boundary

Use `rkyv` only for read-mostly local caches, archive indexes, replay indexes,
and far-field summary blobs where validation and format options are frozen in
canon.
