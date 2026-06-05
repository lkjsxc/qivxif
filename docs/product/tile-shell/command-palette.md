# Command Palette

## Purpose

The command palette exposes tile, graph, editor, media, admin, and profile
actions without requiring hidden UI gestures.

## Commands

- Open node.
- Create text node.
- Open Graph Map.
- Create Graph Map.
- Split pane.
- Stack tab.
- Maximize pane.
- Add current node to Graph Map.
- Pin Graph Map node.
- Link visible graph nodes.
- Open sync status.
- Open history.
- Open settings.
- Open publishing tools.
- Open social tools.
- Open media diagnostics.
- Open admin keys.
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
- Palette commands route through the same app action boundary as visible pane buttons.
- Pane-sensitive commands use the active pane context.
- Commands needing input open the pane that owns the form.

## Current Command Routing

| Command | Durable events | Offline |
| --- | --- | --- |
| Create text node | `node.create` | queued |
| Create Graph Map | `node.create` | queued |
| Split pane | `node.create`, `edge.create`, `tile.layout_set` | queued |
| Stack tab | `node.create`, `edge.create`, `tile.layout_set` | queued |
| Maximize pane | `tile.layout_set` | queued |
| Add node to Graph Map | `edge.create` | queued |
| Pin Graph Map node | `graph_map.item_place` | queued |
| Link visible graph nodes | `edge.create` | queued |
| Open graph | none | available |
| Open sync status | none | available |
| Open history | none | available |
| Open settings | none | available |
| Open publishing tools | none | available |
| Open social tools | none | available |
| Open media diagnostics | none | available |
| Open admin keys | none | requires admin |
| Flush queue | none | available when signed in |
