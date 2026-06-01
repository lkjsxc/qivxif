# Graph Composition

## Purpose

Boards let users create, link, arrange, traverse, and publish graph nodes without thinking in directories.

## Behavior

- Create a `graph_board` node.
- Add existing nodes to a board.
- Create text, post, tag, topic, and media nodes from a board.
- Connect nodes with typed edges.
- Open related nodes in adjacent panes.
- Save board layout offline.
- Sync board events.

## Constraint

Board layout is graph data, not one opaque local blob.

## Durable Shape

- The board is a `graph_board` node.
- Each visible placement is a `board_item` node.
- The board item metadata stores `item_node_id`, `x`, `y`, and `position_key`.
- A `placed_on_board` edge links the board item to the board.
- A `contains` edge links the board item to the displayed graph node.
- Moving an item creates a new placement event.
- The visible position is the active placement selected by accepted relation
  edges and deterministic ordering metadata.

## Event Shape

- `board.item_place` creates or supersedes one visible placement.
- `edge.create` with `placed_on_board` links the placement to the board.
- `edge.create` with `contains` links the placement to the displayed node.
- `edge.tombstone` or `supersedes` relation events remove older active
  placements from the visible projection.

## Ordering

- `position_key` is the primary deterministic ordering key.
- `ordinal` may be used where a board owner needs a compact sequence.
- Store acceptance order is a tie-breaker only when documented.
- Wall-clock time never determines visible order.
