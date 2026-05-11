# Storage References

## Scope

This file stores persistence research. Schema changes require an owner decision.

## Findings

- redb fits local authoritative hot state.
- Object storage fits snapshots, replays, and large archives.
- Generated terrain should be disposable.
- Persisted edits are authoritative chunk-scoped overlays in the initial slice.
- `rkyv` fits validated read-mostly archives and client-local caches, not
  client-trusted gameplay truth.
- Backup and restore should be treated as product behavior because terrain edits
  are persistent and consequential.

## Current Status

The initial slice stores chunk-scoped edit overlays in the redb `sections`
table. The intended deeper-world move is true `SectionCoord { x, y, z }`
storage behind a schema epoch decision.
