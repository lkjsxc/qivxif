# Product Shape

## Surfaces

- Tiled workspace with split panes and tab stacks.
- Graph navigation pane.
- Text editor and Markdown preview panes.
- Blog draft and public route surfaces.
- Social post, feed, reply, and moderation surfaces.
- Sync status and cache diagnostics panes.
- `kjxlkj` board for direct graph composition.

## Durable Model

- Nodes represent durable entities.
- Edges represent durable relationships.
- Operations represent durable changes.
- Commit groups bundle meaningful user actions.
- Snapshots accelerate replay without replacing history.

## Local-First Behavior

The client stores local operations before network delivery. UI projections update from those operations and later reconcile with server-accepted operations.
