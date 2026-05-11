# Coordinate Model

## Status

- Status: implemented for block positions and chunk coordinates.
- Owner: `crates/qivxif-core` and `crates/qivxif-world`.

## Implemented Types

| Type | Fields | Owner |
| --- | --- | --- |
| `BlockPos` | `x: i32`, `y: i32`, `z: i32` | `qivxif-core` |
| `ChunkCoord` | `x: i32`, `z: i32` | `qivxif-core` |

## Implemented Rules

- `CHUNK_EDGE` is `8`.
- `chunk_coord(pos)` uses Euclidean division for `x` and `z`.
- Negative block coordinates map deterministically to negative chunk coordinates.
- `in_chunk(pos, coord)` uses the same Euclidean chunk mapping.
- Current chunk generation iterates local `x`, `y`, and `z` in `0..CHUNK_EDGE`.

## Persistence Key Link

- Current overlay keys are chunk-based.
- Key shape: `section/{chunk_x}/{chunk_z}`.
- Storage details are owned by [../persistence/schema-contracts.md](../persistence/schema-contracts.md).

## Not Implemented

- `SectionCoord { x, y, z }`.
- Vertical section ownership.
- Region coordinate partitioning.
