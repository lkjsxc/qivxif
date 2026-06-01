# Tiled Tabs

## Model

- A workspace has one root tile.
- A tile is either a split or a tab stack.
- A tab references one pane.
- A pane can view a graph node, a feed, settings, history, or diagnostics.

## Behavior

- Dragging a tab to a side creates a split.
- Dragging a tab to a tab strip creates a stack.
- Maximize hides sibling tiles without destroying layout.
- Closing the last tab removes its stack.
- Layout is stored as graph records and operations.
- Offline restore must work from the local cache.

## Constraints

- Multiple panes may view the same node.
- Dirty state is visible at pane and tab level.
- Conflicts need pane-level affordances.
