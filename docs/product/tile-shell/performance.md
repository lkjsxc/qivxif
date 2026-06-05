# Tile Shell Performance

## Purpose

The tile shell must feel immediate. Direct manipulation is never blocked by
storage, sync, WASM cold load, server reachability, or full-app recomputation.

## Budgets

| Path | Budget |
| --- | --- |
| First shell paint | Static header and tile frame render without server dependency. |
| Tab focus | Visual active state changes in the same frame. |
| Drag pointermove | Normal frame work stays under 4ms. |
| Resize pointermove | Normal frame work stays under 4ms. |
| Common reducer command | Under 1ms on normal layouts. |
| Large reducer fixture | Handles 100 panes without visible delay. |

## Direct Manipulation Rules

- Pointermove updates ephemeral render state only.
- No SQLite worker call runs during pointermove, dragover, tab click, or keydown focus.
- No network call runs during direct manipulation.
- WASM service startup cannot block shell rendering or pointer paths.
- Geometry is measured at drag start and refreshed only on scroll or resize.
- Drag overlays update through `requestAnimationFrame`.
- Durable layout persistence runs after accepted commands, drop, or resize end.
- Persistence is queued and debounced when repeated commands occur.

## Render Rules

- Svelte blocks are keyed by stable pane ids.
- Pane bodies are not recreated on tab focus.
- Derived selectors avoid full workspace rerender for one pane change.
- Panes, tab rails, and tab bodies use CSS containment where it helps.
- Long pane content wraps or scrolls vertically inside the pane.

## Reducer Fixtures

Reducer tests cover layouts with these pane counts:

- 1 pane.
- 10 panes.
- 50 panes.
- 100 panes.
- 300 panes.

Each fixture validates no duplicate pane ids, valid active pane ids, non-empty
split children, matching weights, positive weight totals, and no DOM ids.

## Diagnostics

Performance failures are product bugs. Diagnostics should show when the shell is
waiting on storage startup, sync, WASM, or resource planning, but those waits do
not delay focus, drag, resize, reorder, split, stack, maximize, restore, or close.
