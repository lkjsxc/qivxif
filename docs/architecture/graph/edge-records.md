# Edge Records

## Fields

- `id`
- `from_node`
- `to_node`
- `kind`
- `created_by`
- `created_at`
- `metadata_map`
- `tombstone`

## Rules

- Edges are first-class durable facts.
- Edges are not nested arrays inside nodes.
- Forward and reverse indexes are maintained on write.
- Direct edge creation routes receive `event_id`, `actor_seq`, `edge_id`,
  endpoints, `kind`, and `metadata_map`.
- Edge acceptance writes the event record, edge record, endpoint indexes, and
  target indexes atomically.
- Edge tombstones preserve the edge record for history and repair.
