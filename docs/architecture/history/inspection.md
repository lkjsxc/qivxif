# History Inspection

## Purpose

History inspection lets an authorized viewer see the accepted operations that changed a node.

## Route

`GET /api/nodes/{node_id}/history`

Query parameters:

- `limit`: maximum operation summaries, default `50`, maximum `200`.

Success payload:

- `node_id`
- `operations`

Each operation summary includes:

- `op_id`
- `actor_id`
- `actor_seq`
- `scope`
- `kind`
- `target_node_ids`
- `payload_hash`
- `received_at_server`

## Rules

- Server ACL is checked against the target node before any operation leaves the store.
- Operation order follows server cursor acceptance order.
- Payload bytes are not exposed by the first history route.
- Text restore and edit payload details are inspected through text projections and later diff views.
- Tombstoned nodes remain queryable by authorized viewers.
