# Styles

Authored CSS is split by browser surface concern.

## Contents

- [base.css](base.css): variables, root sizing, and global defaults.
- [shell.css](shell.css): header, tile grid, pane, tab rail, and drag overlays.
- [controls.css](controls.css): buttons, forms, command palette, and content controls.

## Rules

- The document body must not scroll horizontally.
- Pane bodies own local vertical scroll.
- Tab rails stay one row and scroll horizontally.
- Long IDs and user text must wrap inside pane content.
