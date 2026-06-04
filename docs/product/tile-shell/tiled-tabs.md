# Tiled Tabs

## Model

- The first visible product surface is the header plus recursive tile grid.
- The app shell has one root tile.
- A tile is either an N-way split node or a tab stack.
- A stack has one pane header, one horizontally scrollable tab rail, one plus
  button, one three-dot tile menu, and one pane body region.
- A visible tab instance references one durable pane instance.
- Two tabs may view the same resource, but their visible state is independent.

## Durable Shape

- A `tile_layout` node owns the tile layout.
- Each durable pane instance is a `pane` node unless a later owner doc introduces
  a distinct `tab` node.
- During the current implementation, `pane_node_id` is the visible tab identity.
- `tile_contains_pane` edges link the layout node to pane nodes.
- `pane_views_node` edges link panes to graph targets.
- `tile.layout_set` stores the complete tile tree snapshot for the layout node.
- The tile tree references pane node IDs, never transient DOM IDs.

## Reducer Contract

- Tile reducers are pure functions over a `TileLayout` snapshot.
- Reducers do not read clocks, DOM state, storage, HTTP state, or global mutable state.
- Every reducer command targets a pane ID or the stack that contains it.
- Stack `active` indexes are local to one stack.
- Opening New Tab appends a chooser tab next to the active tab and activates it.
- Selecting a chooser kind converts that same tab ID to the chosen kind.
- Focusing a tab changes only the target stack.
- Splitting a tab creates a sibling stack at the requested edge using smart insertion.
- Resizing a split updates adjacent `sizes` entries and emits `tile.layout_set`.
- Moving a tab to a stack activates it in the target stack.
- Moving a tab to an edge removes it from the source stack, creates a sibling
  stack, and activates the moved tab.
- Closing or moving the last tab out of a stack collapses that stack from the tree.
- Maximizing a pane only sets `maximized_pane_id`; restore clears it.
- Reducers report missing pane IDs. The browser treats invalid drag drops as
  no-ops before queuing events.

## Header Behavior

- Pane headers are compact tabbars.
- The tab rail is visually primary and owns available width.
- The plus button sits beside the rail and immediately opens a real `New Tab` chooser tab.
- The three-dot menu contains split, stack, maximize or restore, and tile close actions.
- Split actions are not exposed as direct pane-header buttons outside the menu.
- The tab close `x` is rendered only on the active tab in each pane.
- Inactive tabs do not expose hidden close buttons in the accessibility tree.
- The active tab scrolls into view when focus changes or a tab moves into a stack.

## Drag Behavior

- Dragging a tab within its rail reorders the stack.
- Dragging a tab to another pane center moves it into that stack.
- Dragging a tab to a pane body edge creates a split at that edge.
- Drop feedback distinguishes center insertion from edge split.
- Drop feedback never covers the tab rail or tile menu row.
- Strip reordering has priority while the pointer remains in the source strip band.
- Coarse pointers pan the tab rail by default and arm dragging only through long press.
- Text selection is suppressed while a tab drag arms or runs.
- Moving a tab activates and focuses it in the target stack.
- Moving the last tab out of a stack removes the source stack.
- Invalid drops are no-ops.

## Body Behavior

- Maximize hides sibling tiles without destroying layout.
- Closing the last tab removes its stack unless it would leave no usable workspace.
- Layout is stored as graph records and events.
- Offline restore must work from the local repository.
- Pane bodies own local vertical scroll.
- Pane body scroll offsets restore from the visible tab instance snapshot.
- Shell content must not create horizontal page scroll.
- Inactive tabs retain logical state through hidden-mounted tab bodies and snapshots.
- Split children expose resize handles between adjacent panes.

## Command Slice

Command buttons create the same durable records as direct manipulation:

- Split creates a pane node, links it to the layout, and appends `tile.layout_set`.
- Stack creates a pane node inside the active tab stack and appends `tile.layout_set`.
- Maximize appends `tile.layout_set` with `maximized_pane_id`.
- Close removes the pane from the tile tree snapshot through `tile.layout_set`.
- Dirty layout events are visible in the sync status pane until accepted.

## Initial Tab Kinds

- Setup
- Welcome
- New Tab
- Graph Node
- Text Node
- Board
- History
- Sync Status
- Settings
- Diagnostics
