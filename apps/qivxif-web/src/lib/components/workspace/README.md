# Workspace Components

## Purpose

Render the compact header, recursive splits, panes, tab rails, drag previews, and
resize handles.

## Allowed Imports

- Controller dispatch callbacks from component props or context.
- `../surfaces/` tab body router.
- `../../domain/` view types and pure resolver helpers.

## Forbidden Imports

- `../../effects/`, `../../storage/`, raw `fetch`, SQL, IndexedDB, OPFS, or
  direct workers.
- Durable event construction inside components.

## Owner Files

- `WorkspaceRoot.svelte`: app shell entry.
- `TileGrid.svelte` and `SplitNode.svelte`: recursive layout rendering.
- `Pane.svelte`, `PaneHead.svelte`, and `PaneTabStack.svelte`: pane shell.
- `TabStrip.svelte` and `TabFrame.svelte`: tab rail interactions.
- `PaneDropLayer.svelte`: drag preview overlay.
- `ResizeHandle.svelte`, `NewTabButton.svelte`, and `TileMenu.svelte`: tile controls.

## Verification

Run web build and offline browser checks after interaction changes.
