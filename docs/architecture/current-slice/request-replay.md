# Request Replay

## Status

- Status: implemented.
- Owner: `apps/qivxif-serverd::session` and `apps/qivxif-serverd::request`.
- Public verification: `request-replay` probe.

## Rule

- Mutating request identifiers are session-scoped replay guards.
- The replay cache lives in `Session::mutating_responses`.

## Applies To

- `PlaceBlock`.
- `FlushPersistence`.

## Behavior

- The first mutating request for an identifier executes normally.
- The first response is stored by request identifier.
- A repeated identifier returns the stored response.
- A repeated `PlaceBlock` does not apply another mutation.
- A repeated `FlushPersistence` does not perform another flush.
- Reusing one identifier for different mutating intents is invalid client behavior.

## Cross-References

- Message fields: [../network/protocol-messages.md](../network/protocol-messages.md).
- Session phases: [../network/session-lifecycle.md](../network/session-lifecycle.md).
- Player terrain intent: [../../product/world/terrain-editing.md](../../product/world/terrain-editing.md).
