# Components

Svelte UI for the workspace shell and product tab surfaces.

## Trees

- [workspace/](workspace/): tile grid, panes, tab strip, drop layer, command palette.
- [surfaces/](surfaces/): setup, feed, editor, sync, and other tab bodies.

## Rules

- Components receive `viewState` and `actions` props from the controller context.
- No direct IndexedDB or fetch calls inside components.
