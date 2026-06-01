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
- Sync board operations.

## Constraint

Board layout is graph data, not one opaque local blob.

## Durable Shape

- The board is a `graph_board` node.
- Each visible placement is a `board_item` node.
- The board item metadata stores `item_node_id`, `x`, `y`, and `placement_seq`.
- A `placed_on_board` edge links the board item to the board.
- A `contains` edge links the board item to the displayed graph node.
- Moving an item creates a new board item placement record.
- The visible position is the highest `placement_seq` for a board and item pair.
