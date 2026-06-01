# Text CRDT

## Facts

- Diamond Types supports plain text CRDT operation logs.
- Its operation log can be replayed to produce document state at a point in history.
- It encodes runs compactly when possible.

## Implication

qivxif uses a CRDT operation log for text nodes and wraps user undo grouping outside the CRDT.
