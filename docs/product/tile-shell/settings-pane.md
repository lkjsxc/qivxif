# Settings Pane

## Purpose

Settings is a first-class product pane. It is not an alias for Diagnostics.

## Visible State

- Account name from the active session when a sync service session exists.
- Actor ID and profile node ID when the user is signed in.
- Online or offline sync state from the browser shell.
- Queued and rejected local event counts.
- Storage mode from the SQLite worker: `opfs`, `memory`, or `unavailable`.
- Storage degraded or unavailable reason when present.
- Local repository inventory counts.
- Server capability list from `/api/server-info` when reachable.
- Last error if the shell has one.

## Controls

- Authenticated users may flush the local event queue.
- A control must call a real action or be absent.
- Diagnostics-only internals stay in Diagnostics.
- Numeric, JSON, string, enum, and boolean settings preserve typed values.
- Invalid setting drafts stay visible with inline error text and are not saved.

## Rules

- Settings reads browser shell state passed into the renderer.
- Settings does not write storage directly.
- Settings does not build accepted events.
- Settings commands go through the app action boundary.
- Sensitive values are masked and are not exposed in debug text.
