# App Shell

## Responsibilities

- Render the compact header and tile frame.
- Start the SQLite worker repository boundary.
- Load local workspace, queue, tab snapshots, and storage diagnostics.
- Check optional sync setup state when the service is reachable.
- Register service worker.
- Start sync actor when a session and service are available.
- Render sync status, settings, diagnostics, graph, editor, board, feed, and
  publishing as tabs.
- Install the app action table that connects UI commands to reducers and effects.

## Rule

The first visible surface is the app shell: one top header and a tile grid. Setup,
login, sync status, diagnostics, graph, editor, board, social, publishing, and
history surfaces are tabs.

## Startup Order

1. Render static header plus tile frame.
2. Start the SQLite worker.
3. Load local workspace and tab snapshots.
4. Render Welcome, Setup, or the restored active tab.
5. Register `/service-worker.js` when available.
6. Fetch optional `/api/setup` and `/api/server-info` when reachable.
7. Start sync actor when setup, login, and network state allow it.
8. Install keyboard shortcuts against the app action table.
9. Show storage and sync diagnostics in Settings and Diagnostics.

## Offline Rule

If `/api/server-info` fails, the shell still opens from cached assets and local
SQLite state. It marks sync offline and does not mark any service-required effect
as accepted.

## Proof Slice Actor Flow

The browser shell owns the first end-to-end graph and text proof through actor
messages:

1. Local store actor records session state when login succeeds.
2. Tile action actor creates a text node command.
3. Local repository writes a dirty `node.create` queue entry before queued count changes.
4. Sync actor sends the entry to `POST /api/nodes` when a session and network are available.
5. Local repository marks the entry accepted only after the response contains an event acceptance.
6. Editor actor writes a dirty `text.restore` or `text.insert` queue entry before showing the edit as queued.
7. Sync actor sends the entry to `POST /api/text/{node_id}/events` when available.
8. Pull and history panes read accepted service state only after accepted events are visible through API responses.

This browser flow uses route-specific durable mutation endpoints until Rust
reducers are shared with the browser. The route endpoints still append event
envelopes on the service side, so no queued event is treated as accepted without
durable service storage.
