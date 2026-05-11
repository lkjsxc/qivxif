# Hot State

## Canon

Use `redb` for authoritative local hot state.

## Stored Data

- World metadata.
- Chunk-scoped edit overlays stored in `sections`.
- Player profiles.
- Bases and claims.
- Skills.
- Market records.

## Rule

Writes happen outside the region tick path.
Restart-sensitive probes force an explicit flush before server restart.
Generated terrain is disposable; overlays are authoritative hot state.
