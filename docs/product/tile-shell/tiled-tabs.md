# Tiled Tabs

## Model

- The first visible product surface is the header plus recursive tile grid.
- The app shell has one root tile.
- A tile is either a split node or a tab stack.
- A stack has one pane header, one horizontally scrollable tab rail, and one
  pane body region.
- A visible tab instance references one durable pane instance.
- A pane can view a graph node, a feed, settings, history, or diagnostics.
- Two tabs may view the same resource, but their visible state is independent.

## Durable Shape

- A `tile_layout` node owns the tile layout.
- Each durable pane instance is a `pane` node unless a later owner doc
  introduces a distinct `tab` node.
- `tile_contains_pane` edges link the layout node to pane nodes.
- `pane_views_node` edges link panes to graph targets.
- `tile.layout_set` stores the complete tile tree snapshot for the layout node.
- The tile tree references pane node IDs, never transient DOM IDs.

## Behavior

- Pane headers contain a tile menu, a new-tab button, and the tab rail.
- The tab rail is one row, scrolls horizontally, and shows fade edges when
  hidden tabs exist.
- The active tab scrolls into view when focus changes or a tab moves into a
  stack.
- Dragging a tab within its rail reorders the stack.
- Dragging a tab to another pane center moves it into that stack.
- Dragging a tab to a pane edge creates a split at that edge.
- Drop feedback distinguishes center insertion from edge split.
- Strip reordering has priority while the pointer remains in the source strip
  band.
- Coarse pointers pan the tab rail by default and arm dragging only through
  long press.
- Text selection is suppressed while a tab drag arms or runs.
- Moving a tab activates and focuses it in the target stack.
- Moving the last tab out of a stack removes the source stack.
- Invalid drops are no-ops.
- Maximize hides sibling tiles without destroying layout.
- Closing the last tab removes its stack.
- Layout is stored as graph records and events.
- Offline restore must work from the local cache.
- Pane bodies own local vertical scroll.
- Shell content must not create horizontal page scroll.
- Inactive tabs retain logical state, even when a later renderer chooses not to
  keep inactive DOM mounted.

## Command Slice

Before drag gestures exist, command buttons create the same durable records:

- Split creates a pane node, links it to the layout, and appends `tile.layout_set`.
- Stack creates a pane node inside the active tab stack and appends `tile.layout_set`.
- Maximize appends `tile.layout_set` with `maximized_pane_id`.
- Close removes the pane from the tile tree snapshot through `tile.layout_set`.
- Dirty layout events are visible in the sync status pane until accepted.

## Constraints

- Multiple panes may view the same node.
- Dirty state is visible at pane and tab level.
- Local dirty state, scroll, draft, selected item, conflict affordance,
  diagnostics, and command context belong to the visible tab instance.
- Shared resource state belongs to graph node, text document, edge, feed, cache,
  and accepted event records.
- Conflicts need pane-level affordances.
- A rejected layout event keeps the local layout dirty and visible.

## Initial Tab Kinds

- Setup
- Welcome
- Graph Node
- Text Node
- Board
- History
- Sync Status
- Settings
- Diagnostics
