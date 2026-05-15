# Pane Host

Owner doc for pane integration.

## Contract

- `PaneId` identifies a pane across layout and session state.
- `PaneKind` is a closed set.
- The host provides bounds, focus, commands, and frame context.
- Panes return effects for workspace changes.

## Rules

- Panes do not own the tile tree.
- Panes do not perform blocking work on the UI thread.
- Panes serialize only their own durable state.
- Pane titles are supplied to tabs and accessibility.
