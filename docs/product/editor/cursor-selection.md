# Cursor And Selection

Owner doc for caret and selection behavior.

## Cursor

- A pane has one active caret by default.
- Horizontal intent is preserved when moving across short lines.
- Movement changes end the current typing group.
- Go-to-line focuses the requested logical line.

## Selection

- A selection has anchor and active ends.
- Shift movement extends the active end.
- Normalized ranges are used for edit operations.
- Selection rendering must be visible in high contrast themes.

## Clipboard

- Copy does not change dirty state.
- Cut and paste are edit transactions.
- Clipboard failures surface as pane-local notices.
