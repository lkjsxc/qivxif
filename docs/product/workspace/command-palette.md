# Command Palette

## Purpose

The command palette exposes workspace actions without requiring hidden UI gestures.

## Commands

- Open node.
- Create text node.
- Create `kjxlkj` board.
- Split pane.
- Stack tab.
- Maximize pane.
- Open sync status.
- Open history.
- Publish draft.
- Create short post.

## Rules

- Commands declare required capability and offline behavior.
- Commands that mutate durable state create operations.
- Unavailable commands explain the missing capability.

## Initial Command Routing

The first browser shell may expose command buttons instead of a palette surface.
Those buttons still use the command contracts:

| Command | Durable operations | Offline |
| --- | --- | --- |
| Create text node | `node.create` | queued |
| Create `kjxlkj` board | `node.create` | queued |
| Split pane | `node.create`, `edge.create`, `workspace.layout_set` | queued |
| Stack tab | `node.create`, `edge.create`, `workspace.layout_set` | queued |
| Maximize pane | `workspace.layout_set` | queued |
| Add node to board | `node.create`, `edge.create` | queued |
| Move board item | `node.create`, `edge.create` | queued |
| Link board nodes | `edge.create` | queued |
