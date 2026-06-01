# Merge

## Rules

- Text nodes use CRDT merge.
- Graph sets use observed-remove semantics.
- Field maps use per-field registers with deterministic tie-breaking.
- Counters use explicit increment and decrement events.
- Publication conflicts produce visible conflict surfaces.
- ACL changes are server-authoritative.
