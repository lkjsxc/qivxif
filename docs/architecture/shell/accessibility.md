# Accessibility

Owner doc for accessibility architecture.

## Required Tree

- Application root.
- Menu and command palette.
- Tile tabs and splitters.
- Explorer tree.
- Editor surface label, path, dirty state, caret position, and selection summary.
- Browser title and policy notices.

## Rules

- Every pane has an accessible title.
- Focus changes update accessibility state.
- High contrast and UI scale are first-class settings.
- Focus is not indicated by color alone.
