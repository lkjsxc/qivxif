# App Shell

## Responsibilities

- Render the compact header and tile frame.
- Check first-run setup state.
- Register service worker.
- Initialize IndexedDB.
- Start sync actor.
- Render sync status as a tab.

## Rule

The first visible surface is the app shell: one top header and a tile grid. Setup, login, sync status, diagnostics, graph, editor, board, social, publishing, and history surfaces are tabs.

## Startup Order

1. Render static header plus tile frame.
2. Fetch `/api/setup`.
3. If setup is required, render Setup as the active tab and do not require login.
4. Fetch `/api/server-info` when reachable.
5. Open IndexedDB.
6. Register `/service-worker.js`.
7. Start sync actor when setup or login state allows it.
8. Render sync status as a tab.

## Offline Rule

If `/api/server-info` fails, the shell still opens from cached assets and marks sync status offline. It does not mark any server-required effect as accepted.

## Proof Slice Actor Flow

The browser shell owns the first end-to-end graph and text proof through actor messages:

1. Login actor posts credentials to `/api/auth/login`.
2. Local store actor records the returned user, actor id, CSRF token, and cookie-backed session state.
3. Tile action actor creates a text node command.
4. Local store actor writes a dirty `node.create` queue entry before the UI increments queued count.
5. Sync actor sends the entry to `POST /api/nodes` when a session and network are available.
6. Local store actor marks the entry accepted only after the response contains an event acceptance.
7. Editor actor writes a dirty `text.restore` or `text.insert` queue entry before showing the edit as queued.
8. Sync actor sends the entry to `POST /api/text/{node_id}/events`.
9. Pull and history panes read server state only after accepted events are visible through API responses.

This browser flow uses route-specific durable mutation endpoints until Rust
reducers are shared with the browser. The route endpoints still append event
envelopes on the server, so no queued event is treated as accepted without
durable server storage.
