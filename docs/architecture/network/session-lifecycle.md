# Session Lifecycle

## Status

- Status: implemented.
- Owner: `apps/qivxif-serverd::session` and `apps/qivxif-serverd::request`.

## Phase State

| Phase fact | Implementation |
| --- | --- |
| New session starts without hello | `Session::new` sets `hello = false` |
| New session starts without join | `Session::new` sets `joined = false` |
| Accepted hello allows join | `mark_hello`; `can_join` |
| Accepted hello allows ping | `can_ping` |
| Accepted join allows play requests | `mark_joined`; `can_play` |

## Implemented Request Order

1. Connect over QUIC.
2. Send `Hello`.
3. Send `JoinWorld`.
4. Send `Ping`, `ChunkRequest`, `PlaceBlock`, or `FlushPersistence` according to guards.
5. Disconnect by closing the QUIC connection.

## Guard Matrix

| Guard | Trigger | Durable code |
| --- | --- | --- |
| Build contract present | `Hello` carries empty build contract | `BuildContractMissing` |
| Protocol contract match | `Hello` carries mismatched protocol contract | `ProtocolContractMismatch` |
| Hello before join | `JoinWorld` before accepted `Hello` | `HelloRequired` |
| Hello before ping | `Ping` before accepted `Hello` | `HelloRequired` |
| Join before chunk | `ChunkRequest` before accepted `JoinWorld` | `JoinRequired` |
| Join before mutation | `PlaceBlock` before accepted `JoinWorld` | `JoinRequired` |
| Join before flush | `FlushPersistence` before accepted `JoinWorld` | `JoinRequired` |
| Decodable request | Malformed wire bytes on a request stream | `BadRequest` |

## Rules

- Session state is separate from region-owned world state.
- Probes assert `Error.code` only.
- `Error.message` is diagnostic text.
- Public message shapes are owned by [protocol-messages.md](protocol-messages.md).
- Player onboarding product flow is owned by [../../product/player/onboarding.md](../../product/player/onboarding.md).
