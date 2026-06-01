# Event Reducers

## Pure Reducers

- `apply_graph_event(state, event) -> Result<state>`
- `project_node(record, edges, viewer) -> NodeProjection`
- `project_tree(root_id, edges, tombstones) -> TreeProjection`
- `reduce_feed(index, event) -> Result<index>`

## Rules

- Reducers do not touch redb.
- Reducers do not call HTTP, read cookies, or read clocks.
- Reducers are deterministic.
- Duplicate event application is idempotent.
- Unknown event kinds are rejected before reducer entry.
- Server reducers are the durable authority.
- Browser reducers are optimistic mirrors only.

## Edge Reducer Rules

- `edge.create` validates that both endpoint nodes exist.
- `edge.create` inserts an edge record and endpoint indexes.
- `edge.tombstone` marks an edge inactive without deleting the record.
- `edge.relate` writes event, edge, and supersession relation indexes only when
  its relation kind is known.
