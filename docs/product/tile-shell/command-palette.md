# Command Palette

## Purpose

The command palette exposes tile and graph actions without requiring hidden UI gestures.

## Commands

- Open node.
- Create text node.
- Create board.
- Split pane.
- Stack tab.
- Maximize pane.
- Add current node to board.
- Move board item.
- Link board nodes.
- Open sync status.
- Open history.
- Open settings.
- Open publishing tools.
- Open social tools.
- Flush queue.

## Rules

- Commands declare required capability and offline behavior.
- Commands that mutate durable state create events.
- Unavailable commands explain the missing capability.
- The palette opens from the header command button.
- The palette opens with `Control+K` or `Meta+K`.
- The search field is focused when the palette opens.
- Filtering matches command label and disabled reason text.
- `Escape` closes the palette.
- Palette commands route through the same app action boundary as visible pane
  buttons.
- Pane-sensitive commands use the active pane context.
- Commands that need user input open the pane that owns the form instead of
  inventing payload values.

## Current Command Routing

The browser shell exposes both command buttons and a palette surface. Both use
the same command contracts:

| Command | Durable events | Offline |
| --- | --- | --- |
| Create text node | `node.create` | queued |
| Create board | `node.create` | queued |
| Split pane | `node.create`, `edge.create`, `tile.layout_set` | queued |
| Stack tab | `node.create`, `edge.create`, `tile.layout_set` | queued |
| Maximize pane | `tile.layout_set` | queued |
| Add node to board | `node.create`, `edge.create` | queued |
| Move board item | `node.create`, `edge.create` | queued |
| Link board nodes | `edge.create` | queued |
| Open graph | none | available |
| Open sync status | none | available |
| Open history | none | available |
| Open settings | none | available |
| Open publishing tools | none | available |
| Open social tools | none | available |
| Flush queue | none | available when signed in |
