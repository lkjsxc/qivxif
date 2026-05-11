# Region Ownership

## Canon

Region actors own mutable world state.

## Rules

- Non-owner services send messages.
- A region validates mutation before applying it.
- Dirty sections are queued for persistence after mutation.
- Flush commands write queued dirty sections before restart-sensitive checks.
- Cross-region work uses explicit handoff messages.

## Initial Slice

The first slice may use one region actor. It must keep the ownership boundary.
