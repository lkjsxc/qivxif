# Vertical Loop

## Status

- Status: implemented.
- Owner: `apps/qivxif-serverd`, `apps/qivxifctl`, `crates/qivxif-probe`.
- Verification: public QUIC probes.

## Public Request Path

1. Probe connects to the server over QUIC.
2. Probe sends `Hello` with non-empty `build_epoch` and matching `protocol_epoch`.
3. Probe sends `JoinWorld`.
4. Probe sends `ChunkRequest` for `ChunkCoord { x: 0, z: 0 }`.
5. Probe sends `PlaceBlock` with a session-scoped `request_id`.
6. Probe sends `FlushPersistence` with a session-scoped `request_id`.
7. Compose restarts the server for persistence checks.
8. Probe sends `ChunkRequest` for the same chunk.
9. Probe expects the placed `BlockCell` in returned cells.

## Implementation Facts

- `qivxif_probe::ProbeClient` opens a new bidirectional stream per request.
- `qivxif_net::send_wire` and `recv_wire` encode messages with postcard.
- `apps/qivxif-serverd::request` translates protocol messages to region calls.
- `crates/qivxif-sim::RegionHandle` owns chunk, mutation, and flush commands.
- `crates/qivxif-storage::WorldStore` persists chunk overlays.

## Acceptance Rules

- Server internals do not appear in public protocol payloads.
- Region actor is the runtime owner of terrain mutation.
- Storage crate is the redb table owner.
- Probes use QUIC; they do not call server internals.
