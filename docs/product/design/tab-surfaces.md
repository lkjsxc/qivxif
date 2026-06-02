# Tab Surfaces

Each tab kind renders inside a pane body. All surfaces follow the Zed-minimal
density rules from [visual-language.md](visual-language.md).

## Setup

- Centered column, max width 420px.
- Owner name and password fields.
- Single primary action: create owner account.
- Error text below the form in danger color.
- No navigation away from `/`.

## Welcome

- Short intro plus action list when authenticated.
- Actions: create text node, create board, open sync, flush queue.
- Node list below actions when nodes exist.
- Login form when session is missing and setup is complete.

## Graph Node

- Node list with kind, title, and ID snippet.
- Open-node form at top.
- Selecting a node opens it in the current pane context.
- IDs use monospace blocks.

## Text Node

- See [surfaces-editor.md](surfaces-editor.md) for editor layout.
- Playwright: `.editor` textarea fills pane; save queues text event.
- Dirty dot on tab frame when draft differs from last accepted text.

## Board

- Active board ID in monospace.
- Item count and placement summary.
- Canvas or list region for board items.
- Add-current-node action when a node is selected elsewhere.

## Feed

- See [surfaces-feed.md](surfaces-feed.md) for card layout.
- Playwright: `article.feed-card` visible when API returns posts.
- Compose textarea and submit at top when authenticated.

## Publishing

- Draft list and publish form.
- Public route preview when a post is selected.
- No fake published URLs.

## History

- Accepted events for current scope.
- Each row: kind, event ID, target summary.
- Scrollable list, compact rows.

## Sync Status

- Online/offline indicator.
- Queue counts: dirty, accepted pending flush, rejected.
- Expandable queue entries with full event detail.
- Flush action when online.

## Settings

- Session info, capabilities, layout summary.
- Scannable sections with headings.
- Logout action at bottom.

## Diagnostics

- Browser capabilities, service worker state, last error.
- Local store stats: events, nodes, layout records.
- Monospace blocks for IDs and cursors.
- Rejected events with reasons.

## Shared Rules

- Content column max width 960px unless board canvas needs full pane width.
- Section headings use `--text-lg`.
- No horizontal pane scroll caused by long IDs; wrap or truncate in chrome only.
- Action buttons opened from a pane target that pane through pane context.
