# Product Shape

## Surfaces

- Header plus tile grid with split tiles and tab stacks.
- Graph navigation pane.
- Graph Map for 2D relationship exploration.
- Text editor and Markdown preview panes.
- Media upload, preview, and attachment surfaces.
- Profile view and edit surfaces.
- Admin invite, key, and audit surfaces.
- Blog draft and public route surfaces.
- Social post, feed, reply, and moderation surfaces.
- Sync status, resource orchestration, and storage diagnostics panes.

## Durable Model

- Nodes represent durable entities.
- Edges represent durable relationships.
- Events represent durable changes.
- Commit groups bundle meaningful user actions.
- Snapshots accelerate replay without replacing history.
- Resource entries explain storage, cache, media, and job state.

## Local-First Behavior

The client stores local events before network delivery. UI projections update
from those events and later reconcile with server-accepted events.
