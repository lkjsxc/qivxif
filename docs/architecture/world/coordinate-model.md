# Coordinate Model

## Terms

- Block: smallest editable unit.
- Section: future fixed cube of blocks addressed by `SectionCoord { x, y, z }`.
- Chunk: horizontal grouping for streaming.
- Region: authoritative simulation ownership area.

## Rule

Coordinates must be deterministic and serializable.
Chunk ownership uses Euclidean division so negative world coordinates map
deterministically to negative chunk coordinates.

## Initial Slice

The current persistence slice stores chunk-scoped edit overlays in the
`sections` table. The table name is reserved for the future section model, but
the current key is chunk-based: `section/{chunk_x}/{chunk_z}`.

Before deeper world work, introduce true `SectionCoord { x, y, z }` ownership
and migrate the overlay key contract in a schema epoch decision.
