# Protocol Codecs

## Status

- Status: implemented for postcard protocol messages.
- Owner: `crates/qivxif-protocol` and `crates/qivxif-net`.

## Implemented Codec Boundary

- `qivxif_protocol::encode` uses `postcard::to_stdvec`.
- `qivxif_protocol::decode` uses `postcard::from_bytes`.
- `qivxif_net::send_wire` serializes with postcard and finishes the send stream.
- `qivxif_net::recv_wire` reads up to 1 MiB and deserializes with postcard.
- Malformed bytes return `ErrorCode::BadRequest` through the public QUIC path.

## Contract Ownership

- `protocol_contract` owns message shape, enum tags, and field meaning.
- `build_contract` owns client and server build identity.
- `world_id` owns persistent world identity.
- A wire-shape change requires an explicit protocol contract decision.

## Rules

- Protocol payloads are schema-bound and non-self-describing.
- Every public message is cataloged in [protocol-messages.md](protocol-messages.md).
- Mutating requests carry request identifiers.
- Authoritative acknowledgements echo request identifiers.
- Error codes are typed protocol values.
- Error message text is diagnostic only.

## Not Implemented

- Public datagram codecs.
- `rkyv` protocol payloads.
- Compatibility shims for retired wire shapes.
