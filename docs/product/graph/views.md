# Views

## View Types

- Neighborhood view.
- Board view.
- Saved topic view.
- Search result view.
- Feed-derived graph view.

## Rules

- Views are projections over graph data.
- Expensive views must have bounded depth or saved query shape.
- A view may be cached, but durable truth remains graph records and events.

## Neighborhood Query

The first neighborhood view is `GET /api/graph/neighborhood`.

- `node_id` chooses the center node.
- `depth` is capped at `3`.
- `limit` is capped at `100` nodes.
- Nodes hidden by ACL are omitted.
- Edges are returned only when both endpoints are visible.
