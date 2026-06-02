# Drag Drop

## Purpose

Tab drag is the direct manipulation path for the same layout commands exposed by
the tile buttons. It must not create a second layout model.

## Drag Payload

- Payload is the source `pane_node_id`.
- MIME type: `application/x-qivxif-pane`.
- Native HTML5 drag and pointer drag share one resolver.

## Drop Outcomes

- Rail target: reorder before or after the hovered tab frame midpoint.
- Pane center: move tab into the target stack and activate it.
- Pane edge: smart-split the target stack and activate the moved tab.
- Last tab leaving a source stack collapses that stack.
- Same insertion slot or self-drop: focus-only no-op.
- Invalid drops do not queue events.
- Accepted drops append one `tile.layout_set` event.

## Pane Regions

Each pane divides into chrome and body:

| Region | DOM | Drop rule |
| --- | --- | --- |
| Full pane | `.pane` | Drop layer extent |
| Chrome | `.pane-head` | Always center |
| Body | `.pane-stack` | Edge zones plus center |
| Source strip band | source `.tab-strip` | Strip-priority reorder |

Edge hit testing uses the **body rect only**. Chrome never triggers edge split.

## Edge Geometry

- Edge fraction: 28% of body width or height.
- Clamped between 56px and 128px.
- Top, bottom, left, and right corridors are mutually exclusive with center.
- Preview overlays use `bodyOffsetTop = bodyRect.top - paneRect.top` so
  highlights never cover the tab rail or tile menu.

## Strip Priority

While the pointer remains in the source pane tab-strip band
(`clientY <= stripBottom`), zone resolution forces **center** reorder and
suppresses accidental edge splits on the target pane.

## Tab Rail Targets

- Resolve `before` or `after` from the horizontal midpoint of the target tab.
- Tab rail targets take precedence while the pointer is over a tab frame.
- Active tab scrolls into view after reorder or move.

## Pointer Constants

| Constant | Value |
| --- | --- |
| Long-press arm | 250ms |
| Coarse cancel distance | 8px |
| Fine drag threshold | 6px |
| Edge fraction | 28% |
| Edge clamp min | 56px |
| Edge clamp max | 128px |

## Pointer Arming

- Fine pointers may use native browser drag after the movement threshold.
- Coarse pointers use `touch-action: pan-x` until long-press arms drag.
- Armed dragging sets `body.tab-drag-armed` and `user-select: none`.
- Pointer capture is used after activation when supported.

## Reducer Boundary

- UI resolves source pane, target pane, zone, and rail insertion index only.
- Controller dispatches pure tile commands before queueing events.
- Rust reducers are the durable authority for the same command shape.

## Architecture Link

- [../../architecture/client/drag-resolver.md](../../architecture/client/drag-resolver.md):
  resolver modules and shared drag state.
