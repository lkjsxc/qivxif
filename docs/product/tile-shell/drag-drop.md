# Drag Drop

## Purpose

Tab drag is the direct manipulation path for the same layout commands exposed by
the tile buttons. It must not create a second layout model.

## Current Target

- A tab drag starts from a durable pane tab in a stack rail.
- The drag payload is the source `pane_node_id`.
- Dropping on another tab in a rail inserts before or after that tab.
- Dropping on a pane center moves the source tab into the target stack.
- Dropping on a pane edge moves the source tab into a new sibling stack.
- The target pane is the active tab of the stack under the pointer.
- Moving the last tab out of a source stack removes that stack.
- Dropping a tab onto itself or into the same insertion slot is a focus-only
  no-op.
- The moved or reordered tab becomes focused in its resulting stack.
- Invalid drops do not queue events.
- Accepted drops append one `tile.layout_set` event.

## Drop Zones

- Top and bottom edge zones use the vertical axis.
- Left and right edge zones use the horizontal axis.
- Center is the remaining pane area.
- A tab rail target resolves to `before` or `after` from the horizontal midpoint
  of the target tab.
- Tab rail targets take precedence while the pointer is over a tab button.
- Edge zones should be large enough to hit intentionally but not capture normal
  center drops.
- The visible pane marks the active drop zone while a tab is over it.
- A target tab marks the active insertion side while a tab is over it.

## Pointer Arming

- Fine pointers may use native browser drag.
- Coarse pointers arm tab dragging only after a long press.
- Moving too far before the long press cancels arming.
- Armed pointer dragging blocks text selection until pointer release or cancel.
- Pointer dragging and native dragging use the same zone resolver.

## Reducer Boundary

- UI modules only resolve source pane, target pane, and drop zone.
- Browser actors call pure tile reducers before queueing events.
- The Rust reducer is the durable authority for the same command shape.
- Store, sync, and API layers only see the resulting `tile.layout_set` event.

## Completion Target

- Native dragging covers mouse and browser-supported drag devices.
- Pointer dragging covers long-press coarse input.
- Rail reorder, center stacking, and edge splitting use one action boundary.
