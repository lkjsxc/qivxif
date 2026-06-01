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
- Direct edge creation routes receive `op_id`, `actor_seq`, `edge_id`, endpoints, `kind`, and `metadata_map`.
- Edge acceptance writes the edge record, operation record, and both endpoint indexes atomically.
