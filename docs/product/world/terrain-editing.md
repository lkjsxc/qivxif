# Terrain Editing

## Rule

Accepted terrain edits are permanent everywhere.

## Safety Model

- Server validates every edit.
- Abuse mitigation belongs to claims, permissions, logs, replay tools, and operator recovery.
- Persistence stores player edits as authoritative overrides.
- Generated terrain remains disposable under unedited sections.

## Implication

The world can accumulate history. Backups and restore drills are mandatory
operations work, not optional polish.
