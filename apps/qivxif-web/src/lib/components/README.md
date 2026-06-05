# Components

## Purpose

Svelte UI for the workspace shell and product tab surfaces.

## Children

- [workspace/](workspace/): tile grid, panes, tab strip, drop layer, command palette.
- [surfaces/](surfaces/): setup, feed, editor, sync, and other tab bodies.

## Allowed Imports

- Plain view state.
- Dispatch callbacks or migration actions supplied by the controller context.
- Pure domain helpers.

## Forbidden Imports

- Raw storage, SQL, OPFS, IndexedDB, direct workers, or direct network transport.
- Optional server implementation modules.

## Verification

Run web build and component-facing browser checks after UI changes.
