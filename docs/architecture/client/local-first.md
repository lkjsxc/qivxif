# Local First

## Stores

- Durable event log in IndexedDB.
- Materialized node projections in IndexedDB.
- Tab-local snapshots in IndexedDB.
- Hot in-memory state for visible panes.
- Derived indexes for graph neighborhoods and feeds.

## Rule

The UI reads projections. Events update projections through reducers.

## Local Queue Records

The browser event queue stores one record per pending durable mutation:

- `id`: event id.
- `kind`: documented event kind.
- `status`: `dirty`, `pending_validation`, `accepted`, or `rejected`.
- `node_id`: target node when the event targets one node.
- `request`: exact JSON body sent to the durable route.
- `route`: HTTP method and path used by the sync actor.
- `created_at`: client timestamp for display only.
- `last_error`: last structured rejection when present.

`created_at` is never a sync cursor. The queue key is the event id.

Login returns `next_actor_seq`. A fresh browser stores `next_actor_seq - 1`
before reserving the first local actor sequence, so a second client for the
same user does not collide with accepted events from another browser.

## Route Flush Lane

The first browser sync actor flushes dirty queue entries through existing durable routes:

- `node.create` uses `POST /api/nodes`.
- `edge.create` uses `POST /api/edges`.
- `text.insert`, `text.delete`, and `text.restore` use `POST /api/text/{node_id}/events`.
- `tile.layout_set` uses `POST /api/tile-layout`.
- `publish.post` uses `POST /api/publish/{node_id}`.
- `publish.unpublish` uses `POST /api/unpublish/{node_id}`.

HTTP sync push remains the batch event-envelope lane for clients that can
produce the full envelope payload. The browser route flush lane is valid only
because the server route creates the same durable event log entry before
returning acceptance.

## Tile Layout And Board Queue Rules

- Layout commands write the local layout snapshot before queue display changes.
- Board placement records are visible locally while dirty.
- Text drafts are stored in `tab_snapshots` by pane ID and survive reload.
- Edge events are flushed after their source and target node events.
- A second client reconstructs board items from accepted graph records.
