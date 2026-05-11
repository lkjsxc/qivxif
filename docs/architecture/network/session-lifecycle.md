# Session Lifecycle

## Phases

1. Connect.
2. Hello.
3. Join world.
4. Request chunks.
5. Send gameplay intent.
6. Receive authoritative outcome.
7. Disconnect or timeout.

## Rejection

- Requests before hello return `HelloRequired`.
- Gameplay requests before join return `JoinRequired`.
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

The public probe asserts codes only. Diagnostic `Error.message` text is not a
durable compatibility surface.

## Rule

Session state is separate from region-owned world state.
Public request and response shapes are owned by
[protocol-messages.md](protocol-messages.md).
