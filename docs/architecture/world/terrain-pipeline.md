# Terrain Pipeline

## Status

- Status: implemented as deterministic toy generation plus overlays.
- Owner: `crates/qivxif-world`.

## Implemented Generation

- `generated_block(pos, seed)` is deterministic.
- Surface formula uses `pos.x`, `pos.z`, and `seed`.
- `pos.y < surface` returns `STONE`.
- `pos.y == surface` returns `GRASS`.
- `pos.y > surface` returns `AIR`.

## Implemented Chunk Assembly

1. Iterate all positions in one chunk volume.
2. Push generated cells only when block is not `AIR`.
3. Apply persisted edit overlays after generation.
4. Replace any existing cell at the edited position.
5. Omit edited cells when the edit block is `AIR`.

## Rule

- Generated terrain is disposable.
- Persisted edits are authoritative overlays.
- Persisted air cells are removal overrides for generated blocks.

## Not Implemented

- Biomes.
- Caves.
- Rivers.
- Decorators.
- Resources.
- Dungeons.
