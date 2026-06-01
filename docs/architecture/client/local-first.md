# Local First

## Stores

- Durable operation log in IndexedDB.
- Materialized node projections in IndexedDB.
- Hot in-memory state for visible panes.
- Derived indexes for graph neighborhoods and feeds.

## Rule

The UI reads projections. Operations update projections through reducers.

## Local Queue Records

The browser operation queue stores one record per pending durable mutation:

- `id`: operation id.
- `kind`: documented operation kind.
- `status`: `dirty`, `pending_validation`, `accepted`, or `rejected`.
- `node_id`: target node when the operation targets one node.
- `request`: exact JSON body sent to the durable route.
- `route`: HTTP method and path used by the sync actor.
- `created_at`: client timestamp for display only.
- `last_error`: last structured rejection when present.

`created_at` is never a sync cursor. The queue key is the operation id.

## Route Flush Lane

The first browser sync actor flushes dirty queue entries through existing durable routes:

- `node.create` uses `POST /api/nodes`.
- `edge.create` uses `POST /api/edges`.
- `text.insert`, `text.delete`, and `text.restore` use `POST /api/text/{node_id}/ops`.
- `workspace.layout_set` uses `POST /api/workspace/layout`.

HTTP sync push remains the batch operation-envelope lane for clients that can produce the full envelope payload. The browser route flush lane is valid only because the server route creates the same durable operation log entry before returning acceptance.

## Workspace And Board Queue Rules

- Layout commands write the local layout snapshot before queue display changes.
- Board placement records are visible locally while dirty.
- Edge operations are flushed after their source and target node operations.
- A second client reconstructs board items from accepted graph records.
