# Region Ownership

## Status

- Status: implemented for one region actor.
- Owner: `crates/qivxif-sim::RegionHandle`.

## Implemented Facts

- `RegionHandle::spawn` starts one Tokio task.
- The actor owns `Region` state.
- Commands are `Chunk`, `PlaceBlock`, and `Flush`.
- Non-owner code calls async methods on `RegionHandle`.
- `place_block` validates block height before mutation.
- Valid mutation updates the dirty chunk overlay.
- `flush` writes dirty chunk overlays through `WorldStore`.
- Dirty state is cleared after flush by draining the map.

## Validation Rule

- Current y bounds are `0..CHUNK_EDGE`.
- Invalid y returns `SimError::InvalidBlockPos`.

## Not Implemented

- Multiple region actors.
- Region partitioning.
- Cross-region transfer.
