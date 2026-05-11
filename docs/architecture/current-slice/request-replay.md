# Request Replay

## Rule

Mutating request identifiers are session-scoped replay guards.

## Applies To

- `PlaceBlock`.
- `FlushPersistence`.

## Behavior

- The first mutating request for an identifier executes normally.
- A repeated identifier returns the first recorded authoritative response.
- A repeated `PlaceBlock` does not apply another mutation.
- A repeated `FlushPersistence` does not perform another flush.

## Rationale

Reliable streams already preserve ordered bytes inside one stream. Request
replay guards protect client retry behavior across newly opened streams in the
same session without expanding the public protocol.
