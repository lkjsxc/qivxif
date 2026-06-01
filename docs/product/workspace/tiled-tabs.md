# Tiled Tabs

## Model

- A workspace has one root tile.
- A tile is either a split or a tab stack.
- A tab references one pane.
- A pane can view a graph node, a feed, settings, history, or diagnostics.

## Durable Shape

- A `workspace_layout` node owns the tiled layout.
- Each durable pane is a `pane` node.
- `workspace_contains_pane` edges link the layout node to pane nodes.
- `pane_views_node` edges link panes to graph targets.
- `workspace.layout_set` stores the complete tile tree snapshot for the layout node.
- The tile tree references pane node IDs, never transient DOM IDs.

## Behavior

- Dragging a tab to a side creates a split.
- Dragging a tab to a tab strip creates a stack.
- Maximize hides sibling tiles without destroying layout.
- Closing the last tab removes its stack.
- Layout is stored as graph records and operations.
- Offline restore must work from the local cache.

## Command Slice

Before drag gestures exist, command buttons create the same durable records:

- Split creates a pane node, links it to the layout, and appends `workspace.layout_set`.
- Stack creates a pane node inside the active tab stack and appends `workspace.layout_set`.
- Maximize appends `workspace.layout_set` with `maximized_pane_id`.
- Close removes the pane from the tile tree snapshot through `workspace.layout_set`.
- Dirty layout operations are visible in the sync status pane until accepted.

## Constraints

- Multiple panes may view the same node.
- Dirty state is visible at pane and tab level.
- Conflicts need pane-level affordances.
- A rejected layout operation keeps the local layout dirty and visible.
