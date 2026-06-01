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

## Proof Slice Actor Flow

The browser shell owns the first end-to-end graph and text proof through actor messages:

1. Login actor posts credentials to `/api/auth/login`.
2. Local store actor records the returned user, actor id, CSRF token, and cookie-backed session state.
3. Workspace actor creates a text node command.
4. Local store actor writes a dirty `node.create` queue entry before the UI increments queued count.
5. Sync actor sends the entry to `POST /api/nodes` when a session and network are available.
6. Local store actor marks the entry accepted only after the response contains an operation acceptance.
7. Editor actor writes a dirty `text.restore` or `text.insert` queue entry before showing the edit as queued.
8. Sync actor sends the entry to `POST /api/text/{node_id}/ops`.
9. Pull and history panes read server state only after accepted operations are visible through API responses.

This browser flow uses route-specific durable mutation endpoints until Rust reducers are shared with the browser. The route endpoints still append operation envelopes on the server, so no queued operation is treated as accepted without durable server storage.
