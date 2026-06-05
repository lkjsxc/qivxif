# Edge Events

## Purpose

Edges are graph data. Edge events are the accepted durable facts that create,
relate, supersede, order, or tombstone edge records.

## Edge Create

`edge.create` creates one `EdgeRecord`:

- `edge_id`
- `from_node`
- `to_node`
- `kind`
- `metadata_map`

Acceptance writes the event, edge record, endpoint indexes, and target indexes
in one transaction.

## Edge Tombstone

`edge.tombstone` marks an edge inactive and preserves history:

- The edge remains inspectable.
- Endpoint indexes remain repairable.
- Tree and Graph Map projections ignore tombstoned relation edges unless the owner
  doc explicitly asks for history display.

## Edge Relate

`edge.relate` is reserved for concrete relation needs between an edge and
another event or edge. Prefer `edge.create` and `edge.tombstone` until a use case
requires relation events.

## Relation Kinds

Initial edge-backed relation kinds include:

- `contains`
- `parent_of`
- `ordered_child`
- `reply_to`
- `references`
- `supersedes`
- `tile_contains_pane`
- `pane_views_node`
- `placed_on_graph_map`
