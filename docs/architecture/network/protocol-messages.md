# Protocol Messages

## Owner

This file owns public qivxif protocol messages for protocol epoch `1`.
`qivxif-protocol` owns the Rust definitions and postcard encoding.

## Lanes

| Lane | Current use | Owner |
| --- | --- | --- |
| Reliable bidirectional stream | Hello, join, ping, chunk, mutation, flush, and errors | Protocol crate and server session code |
| Datagram | Reserved for later latest-wins state | No public payloads in epoch `1` |

## Request Identifiers

- Mutating client requests carry `request_id: u64`.
- Acknowledgements echo the same identifier.
- `PlaceBlock` and `FlushPersistence` require request identifiers.
- Non-mutating bootstrap and query messages do not carry request identifiers.
- Request identifiers are scoped to one server session.
- Repeated mutating identifiers return the first authoritative response without
  applying the mutation or flush again.
- Reusing a mutating identifier for a different intent is invalid client
  behavior; the server still preserves the first response as session truth.

## Client Messages

| Message | Fields | Phase | Response |
| --- | --- | --- | --- |
| `Hello` | `build_epoch`, `protocol_epoch` | After connect | `HelloOk` or `Error` |
| `JoinWorld` | `player` | After hello | `Joined` or `Error` |
| `Ping` | `nonce` | After hello | `Pong` or `Error` |
| `ChunkRequest` | `coord` | After join | `Chunk` or `Error` |
| `PlaceBlock` | `request_id`, `pos`, `block` | After join | `MutationAck` or `Error` |
| `FlushPersistence` | `request_id` | After join | `FlushAck` or `Error` |

## Server Messages

| Message | Fields | Rule |
| --- | --- | --- |
| `HelloOk` | `session_id`, `world_epoch`, `caps` | Opens the session for join and ping |
| `Joined` | `player` | Echoes accepted player label |
| `Pong` | `nonce` | Echoes ping nonce |
| `Chunk` | `coord`, `cells` | Returns generated cells plus persisted overlays |
| `MutationAck` | `request_id`, `cell` | Echoes mutation id and authoritative cell |
| `FlushAck` | `request_id` | Echoes flush id after queued overlays are written |
| `Error` | `code`, `message` | Code is durable; message is diagnostic |

## Capability Fields

| Field | Local Compose value | Meaning |
| --- | --- | --- |
| `reliable_streams` | `true` | Public protocol uses reliable streams |
| `datagrams` | `false` | No public datagram payloads in epoch `1` |
| `persistent_mutations` | `true` | Place and flush persist chunk-scoped overlays |

## Error Codes

| Code | Meaning |
| --- | --- |
| `BadRequest` | Decode or request framing failed before session mutation |
| `BuildEpochMissing` | Client or server build epoch was empty |
| `ProtocolEpochMismatch` | Client protocol epoch differs from server |
| `HelloRequired` | Request requires hello first |
| `JoinRequired` | Request requires join first |
| `ChunkError` | Chunk load or generation failed |
| `MutationError` | Mutation validation or queueing failed |
| `FlushError` | Persistence flush failed |

Error codes are durable protocol outcomes. Error messages are diagnostic text
for operators and probes must not assert exact message wording.

Malformed wire bytes must return `BadRequest` through the public QUIC path and
must not advance the session phase.

## Encoding Rules

- Public messages round-trip through postcard.
- Enum tags and field meanings are owned by `protocol_epoch`.
- No retired wire-shape shims exist.
