# Terrain Editing

## Scope

This file owns durable terrain edit behavior.

## Rule

Accepted terrain edits are durable player-visible world changes.

## Safety Model

- Server validates every edit.
- Abuse mitigation belongs to claims, permissions, logs, replay tools, and operator recovery.
- Persistence stores player edits as authoritative overrides.
- Generated terrain remains disposable under unedited sections.

## Implication

The world can accumulate history. Backups and restore drills are mandatory
operations work, not optional polish.

## Cross-References

- Claim permissions are defined in [../gameplay/bases-claims.md](../gameplay/bases-claims.md).
- First-person precision use cases are defined in [../player/camera-controls.md](../player/camera-controls.md).
