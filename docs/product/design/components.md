# Components

## App Header

- Height: 40–44px.
- Three zones: brand, status strip, actions.
- Status strip shows sync state, queue count, online/offline.
- Actions: command trigger, user/session indicator.
- Border bottom: `--q-border`.
- Background: `--q-bg-deep`.

## Pane Chrome

- Contains tile menu, tab strip, new-tab button.
- Height: 36–40px.
- Background: `--q-surface-raised`.
- No title bar separate from tab rail.
- Tile menu opens split, stack, maximize, close for the clicked pane.

## Tab Frame

- One row per tab in the rail.
- Max width ~180px, truncated title with ellipsis.
- Active tab: raised surface plus `--q-border-strong`; text stays `--q-text`.
- Close button is a separate hit target, not the whole tab.
- Dirty tabs show a dot or asterisk before the title.
- `data-pane-id` and `data-tab-kind` attributes for tests.

## Tab Stack

- Fills pane below chrome.
- All tabs stay mounted; inactive tabs are hidden with `aria-hidden`.
- Active tab body uses `.tab-body` with local vertical scroll.
- No horizontal scroll inside the stack region.

## Drop Layer

- Covers full pane for hit testing.
- Preview overlays use body rect offset below chrome.
- Edge preview shows half-body highlight on the target edge.
- Center preview covers the body region only.

## Resize Handle

- Placed between N-way split children.
- Hit target: 4px.
- Visible line on hover/active.
- Cursor: `col-resize` or `row-resize` by split axis.

## Command Palette

- Opens from header or `Control+K` / `Meta+K`.
- Centered overlay, max width 560px.
- Backdrop dim at ~38% black.
- Selected command uses accent border.
- Search field focused on open.

## Forms

- Label above field, `--q-muted` label color.
- Input background `--q-surface-raised`, border `--q-border`.
- Full width inside pane content column.
- Primary submit uses accent fill.

## Status Chips

- Inline in header or sync tab.
- Small caps or uppercase optional for sync state only.
- Colors: muted default, warning for queued, danger for rejected.

## Queue Entry

- Monospace event ID on first line.
- Kind and status on second line.
- Rejection reason in `--q-danger` when present.
- `overflow-wrap: anywhere` for long IDs.
