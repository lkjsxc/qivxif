# Replication

## Status

- Status: minimal chunk response only.
- Owner: `apps/qivxif-serverd::request` and `crates/qivxif-world`.

## Implemented Facts

- `ChunkRequest` returns one `ServerMsg::Chunk` response.
- The response includes `coord` and `cells`.
- Cells come from deterministic generation plus persisted overlays.
- There is no continuous entity replication loop.
- There is no client prediction protocol.

## Not Implemented

- Entity relevance selection.
- Movement prediction correction.
- Inventory replication.
- Combat replication.
- Datagram state deltas.

## Rule

- Replication is not mutation ownership.
- Region actors still own accepted mutation.
