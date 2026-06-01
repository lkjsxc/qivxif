# Operation Reducers

## Pure Reducers

- `apply_graph_op(state, op) -> Result<state>`
- `project_node(record, edges, viewer) -> NodeProjection`
- `reduce_feed(index, op) -> Result<index>`

## Rules

- Reducers do not touch redb.
- Reducers are deterministic.
- Duplicate operation application is idempotent.
- Unknown operation kinds are rejected until documented.
