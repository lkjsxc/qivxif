# Storage References

## Findings

- redb fits local authoritative hot state.
- Object storage fits snapshots, replays, and large archives.
- Generated terrain should be disposable.
- Persisted edits are authoritative chunk-scoped overlays in the initial slice.

## Current Status

The initial slice stores chunk-scoped edit overlays in the redb `sections`
table. The intended deeper-world move is true `SectionCoord { x, y, z }`
storage behind a schema epoch decision.
