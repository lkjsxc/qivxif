# Hot State

## Status

- Status: implemented for world metadata and chunk edit overlays.
- Owner: `crates/qivxif-storage::WorldStore`.

## Implemented Stored Data

| Data | Location | Encoding |
| --- | --- | --- |
| `WorldMeta` | `meta` table key `world` | postcard |
| Chunk edit overlays | `sections` table key `section/{x}/{z}` | postcard `Vec<BlockCell>` |

## Implemented Behavior

- `WorldStore::open(root, seed)` creates the data directory.
- `WorldStore::open` creates or opens `world.redb`.
- Existing `WorldMeta` wins over a new seed on reopen.
- `put_block` maps block position to chunk coordinate.
- `put_block` replaces the existing overlay cell at the same position.
- `put_chunk` writes a complete chunk overlay.
- `load_chunk` returns an empty vector when no overlay exists.

## Durability Facts

- Table bootstrap uses redb immediate durability.
- Overlay writes use redb immediate durability.
- Metadata creation uses redb immediate durability.
- Restart-sensitive probes call `FlushPersistence` before restart.

## Not Implemented

- Player profiles.
- Bases and claims.
- Skills.
- Market records.
- Mail.
