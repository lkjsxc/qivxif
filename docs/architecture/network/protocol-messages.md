# Protocol Messages

## Status

- Status: implemented for protocol contract `postcard-reliable-streams`.
- Owner: `crates/qivxif-protocol`.
- Encoding: postcard.

## Request Identifiers

- Type alias: `RequestId = u64`.
- `PlaceBlock` carries `request_id`.
- `FlushPersistence` carries `request_id`.
- Acknowledgements echo the same identifier.
- Identifiers are scoped to one server session.
- Replay behavior is owned by [../current-slice/request-replay.md](../current-slice/request-replay.md).

## Client Messages

| Message | Fields | Phase | Response |
| --- | --- | --- | --- |
| `Hello` | `build_contract`, `protocol_contract` | After connect | `HelloOk` or `Error` |
| `JoinWorld` | `player` | After hello | `Joined` or `Error` |
| `Ping` | `nonce` | After hello | `Pong` or `Error` |
| `ChunkRequest` | `coord` | After join | `Chunk` or `Error` |
| `PlaceBlock` | `request_id`, `pos`, `block` | After join | `MutationAck` or `Error` |
| `FlushPersistence` | `request_id` | After join | `FlushAck` or `Error` |

## Server Messages

| Message | Fields | Rule |
| --- | --- | --- |
| `HelloOk` | `session_id`, `world_id`, `caps` | Opens the session for join and ping |
| `Joined` | `player` | Echoes accepted player label |
| `Pong` | `nonce` | Echoes ping nonce |
| `Chunk` | `coord`, `cells` | Returns generated visible cells plus persisted overlays |
| `MutationAck` | `request_id`, `cell` | Echoes mutation id and authoritative cell |
| `FlushAck` | `request_id` | Echoes flush id after dirty overlays are written |
| `Error` | `code`, `message` | Code is durable; message is diagnostic |

## Capability Fields

| Field | Current value | Meaning |
| --- | --- | --- |
| `reliable_streams` | `true` | Public protocol uses reliable streams |
| `datagrams` | `false` | No public datagram payloads exist |
| `persistent_mutations` | `true` | Place and flush persist chunk overlays |

## Error Codes

| Code | Meaning |
| --- | --- |
| `BadRequest` | Decode or request framing failed before session mutation |
| `BuildContractMissing` | Client or server build contract was empty |
| `ProtocolContractMismatch` | Client protocol contract differs from server |
| `HelloRequired` | Request requires hello first |
| `JoinRequired` | Request requires join first |
| `ChunkError` | Chunk load or generation failed |
| `MutationError` | Mutation validation or queueing failed |
| `FlushError` | Persistence flush failed |

## Encoding Rules

- Public messages round-trip through postcard.
- Enum tags and field meanings are owned by `protocol_contract`.
- No retired wire-shape shims exist.
