# Text CRDT

## Facts

- Diamond Types supports plain text CRDT edit logs.
- Its edit log can be replayed to produce document state at a point in history.
- It encodes runs compactly when possible.

## Implication

qivxif stores text edit events that carry CRDT edit payloads and wraps user
undo grouping outside the CRDT.
