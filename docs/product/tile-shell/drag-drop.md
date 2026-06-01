# Drag Drop

## Purpose

Tab drag is the direct manipulation path for the same layout commands exposed by
the tile buttons. It must not create a second layout model.

## Current Target

- A tab drag starts from a durable pane tab in a stack rail.
- The drag payload is the source `pane_node_id`.
- Dropping on a pane center moves the source tab into the target stack.
- Dropping on a pane edge moves the source tab into a new sibling stack.
- The target pane is the active tab of the stack under the pointer.
- Moving the last tab out of a source stack removes that stack.
- Moving within the same target pane is a focus-only no-op.
- Invalid drops do not queue events.
- Accepted drops append one `tile.layout_set` event.

## Drop Zones

- Top and bottom edge zones use the vertical axis.
- Left and right edge zones use the horizontal axis.
- Center is the remaining pane area.
- Edge zones should be large enough to hit intentionally but not capture normal
  center drops.
- The visible pane marks the active drop zone while a tab is over it.

## Reducer Boundary

- UI modules only resolve source pane, target pane, and drop zone.
- Browser actors call pure tile reducers before queueing events.
- The Rust reducer is the durable authority for the same command shape.
- Store, sync, and API layers only see the resulting `tile.layout_set` event.

## Follow-On Target

- Pointer dragging and native dragging must share the same drop-zone resolver.
- Coarse pointers arm dragging through long press.
- Strip-local reorder uses the same source and target pane identifiers.
