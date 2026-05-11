# World Architecture

Use this subtree for implemented world data facts.

## Current Implementation

- Blocks are addressed by `BlockPos { x, y, z }`.
- Chunks are addressed by `ChunkCoord { x, z }`.
- `CHUNK_EDGE` is `8`.
- Generated block IDs are `AIR = 0`, `STONE = 1`, and `GRASS = 2`.
- Chunk cells include visible non-air generated cells plus persisted overlays.

## Child Index

- [coordinate-model.md](coordinate-model.md): coordinates and chunk mapping.
- [terrain-pipeline.md](terrain-pipeline.md): implemented generation and overlay order.
- [chunk-streaming.md](chunk-streaming.md): current chunk request behavior.
- [lod-and-summaries.md](lod-and-summaries.md): dormant LOD notes.
- [safe-spawn-and-teleport.md](safe-spawn-and-teleport.md): dormant travel notes.
