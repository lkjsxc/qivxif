# History Inspection

## Purpose

History inspection lets an authorized viewer see the accepted events that
changed a node.

## Route

`GET /api/nodes/{node_id}/history`

Query parameters:

- `limit`: maximum event summaries, default `50`, maximum `200`.

Success payload:

- `node_id`
- `events`

Each event summary includes:

- `event_id`
- `actor_id`
- `actor_seq`
- `scope`
- `kind`
- `target_node_ids`
- `payload_hash`
- `received_at_server`

## Rules

- Server ACL is checked against the target node before any event leaves the store.
- Event order follows server cursor acceptance order.
- Payload bytes are not exposed by the first history route.
- Text restore and edit payload details are inspected through text projections and later diff views.
- Tombstoned nodes remain queryable by authorized viewers.
