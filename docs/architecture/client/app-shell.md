# App Shell

## Responsibilities

- Load the tiled workspace.
- Register service worker.
- Initialize IndexedDB.
- Start sync actor.
- Render sync status.

## Rule

The first visible surface is the workspace.

## Startup Order

1. Render static workspace frame.
2. Fetch `/api/server-info`.
3. Open IndexedDB.
4. Register `/service-worker.js`.
5. Start sync actor.
6. Render sync status pane.

## Offline Rule

If `/api/server-info` fails, the shell still opens from cached assets and marks sync status offline. It does not mark any server-required effect as accepted.
