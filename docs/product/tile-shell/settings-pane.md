# Settings Pane

## Purpose

Settings is a first-class product pane. It is not an alias for Diagnostics.

## Visible State

- Account name from the active session.
- Actor ID and profile node ID when the user is signed in.
- Online or offline state from the browser shell.
- Queued and rejected local event counts.
- Server capability list from `/api/server-info`.
- Last error if the shell has one.

## Controls

- Authenticated users may flush the local event queue.
- A control must call a real action or be absent.
- Diagnostics-only internals stay in Diagnostics.

## Rules

- Settings reads browser shell state passed into the renderer.
- Settings does not write IndexedDB directly.
- Settings does not build accepted events.
- Settings commands go through the app action boundary.
