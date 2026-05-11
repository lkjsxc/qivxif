# Coordinate Model

## Terms

- Block: smallest editable unit.
- Section: fixed cube of blocks.
- Chunk: horizontal grouping for streaming.
- Region: authoritative simulation ownership area.

## Rule

Coordinates must be deterministic and serializable.
Chunk ownership uses Euclidean division so negative world coordinates map
deterministically to negative chunk coordinates.
