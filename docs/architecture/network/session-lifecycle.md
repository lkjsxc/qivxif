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

## Rule

Session state is separate from region-owned world state.
