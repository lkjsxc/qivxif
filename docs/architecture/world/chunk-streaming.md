# Chunk Streaming

## Status

- Status: implemented as direct chunk request and response.
- Owner: `ClientMsg::ChunkRequest`, `ServerMsg::Chunk`, and `qivxif_world::chunk_cells`.

## Implemented Behavior

- Client or probe sends `ChunkRequest { coord }` after join.
- Server asks the region actor for the chunk.
- Region loads persisted overlay cells for the chunk.
- Region merges dirty in-memory cells for the chunk.
- World generation creates visible generated cells.
- Overlays replace generated cells at the same position.
- Response is `Chunk { coord, cells }`.

## Not Implemented

- Interest management by player position.
- Visual radius selection.
- Interaction radius prioritization.
- Chunk subscriptions.
- Decorative data tiers.

## Product Links

- World expectations: [../../product/world/README.md](../../product/world/README.md).
- Camera expectations: [../../product/player/camera-controls.md](../../product/player/camera-controls.md).
