# Workspace Layout

The active workspace is the server/probe slice. Client, renderer, auth,
gameplay, and mobile crates are added only after their owner docs are active.

## Apps

- `qivxif-serverd`: authoritative server.
- `qivxifctl`: quality and probe command.

## Crates

- `qivxif-core`.
- `qivxif-protocol`.
- `qivxif-net`.
- `qivxif-world`.
- `qivxif-sim`.
- `qivxif-storage`.
- `qivxif-quality`.
- `qivxif-probe`.

## Planned Areas

- Shared client core.
- Desktop and mobile platform shells.
- Renderer family.
- Authentication and identity.
- Gameplay systems.
