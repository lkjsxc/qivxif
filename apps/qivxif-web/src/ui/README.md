# UI Modules

This directory owns replaceable DOM rendering and gesture code.

## Contents

- `shell.ts`, `header.ts`, and `tile-tree.ts`: app shell, header, split tree,
  pane header, tab rail, and tab body composition.
- `tab-content.ts`: tab-kind renderer selection.
- `tab-rail.ts`, `tab-drag.ts`, `drop-layer.ts`, and `drop-resolver.ts`: tab
  movement, reordering, edge split targeting, and overlays.
- Product surface files render setup, sync, settings, board, publish, and social panes.

## Rules

- UI modules render current state and emit action callbacks.
- UI modules do not import HTTP clients or IndexedDB adapters.
- Pane bodies own vertical scroll.
- The shell must not create horizontal page scroll.
- Data attributes expose pane IDs for diagnostics and browser tests.
