# Browser Domain

## Purpose

Pure reducers and deterministic helpers for workspace state.

## Allowed Imports

- Other `../domain/` modules.
- Plain DTO types with no side effects.

## Forbidden Imports

- Svelte components.
- `../effects/`, `../storage/`, `../wasm/`, or `../app/` concrete ports.
- DOM, storage, HTTP, workers, random IDs, clocks, or global mutable state.

## Owner Files

- `workspace-state.ts`: initial state shape.
- `workspace-command.ts`: UI command union.
- `tile-tree.ts`: focus, open, close, split, maximize, and restore reducers.
- `tile-move.ts`: move and reorder reducers for dragged tabs.
- `tile-tab-update.ts`: New Tab insertion and conversion helpers.
- `drop-resolver.ts`: pure drop zone resolver.
- `graph-map-view.ts`: pure Graph Map item and edge projection helpers.
- `markdown-preview.ts`: pure Markdown blocks, stats, and search counts.
- `resource-planner.ts`: pure resource protection and action planning.

## Verification

Prefer fixture tests for reducers and run `qivxifctl quality check-lines`.
