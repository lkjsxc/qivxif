# Session Lifecycle

## Phases

1. Connect.
2. Hello.
3. Join world.
4. Request chunks.
5. Send gameplay intent.
6. Receive authoritative outcome.
7. Disconnect or timeout.

Player-facing onboarding flow is owned by
[../../product/player/onboarding.md](../../product/player/onboarding.md).

## Rejection

- `JoinWorld` and `Ping` before accepted hello return `HelloRequired`.
- Chunk, mutation, and flush requests before accepted join return `JoinRequired`.
- Protocol epoch mismatch returns `ProtocolEpochMismatch`.

## Guard Matrix

| Guard | Trigger | Durable code |
| --- | --- | --- |
| Build epoch present | `Hello` carries an empty build epoch | `BuildEpochMissing` |
| Protocol epoch match | `Hello` carries a mismatched protocol epoch | `ProtocolEpochMismatch` |
| Hello before join | `JoinWorld` is sent before accepted `Hello` | `HelloRequired` |
| Hello before ping | `Ping` is sent before accepted `Hello` | `HelloRequired` |
| Join before chunk | `ChunkRequest` is sent before accepted `JoinWorld` | `JoinRequired` |
| Join before mutation | `PlaceBlock` is sent before accepted `JoinWorld` | `JoinRequired` |
| Join before flush | `FlushPersistence` is sent before accepted `JoinWorld` | `JoinRequired` |
| Decodable request | Malformed wire bytes are sent on a request stream | `BadRequest` |

The public probe asserts codes only. Diagnostic `Error.message` text is not a
durable contract surface.

## Rule

Session state is separate from region-owned world state.
Public request and response shapes are owned by
[protocol-messages.md](protocol-messages.md).
