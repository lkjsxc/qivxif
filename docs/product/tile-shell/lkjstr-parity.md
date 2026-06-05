# lkjstr Parity

## Purpose

This file defines what "like lkjstr except colors" means for qivxif.
qivxif copies shell structure and interaction grammar, not Nostr content.

## Reference Components

Inspect these lkjstr files before shell changes:

- `Pane.svelte`.
- `PaneHead.svelte`.
- `TabStrip.svelte`.
- `TabFrame.svelte`.
- `PaneDropLayer.svelte`.
- `TileMenu.svelte`.
- `NewTabButton.svelte`.
- `PaneTabStack.svelte`.
- split node and recursive rendering files.
- drag resolver and shared drag state files.
- workspace store and controller files.

## Copy

- Compact header plus full-height recursive workspace.
- N-way splits with resize handles.
- Tab splitting, stacking, moving, closing, maximizing, and restoring.
- Pane-local vertical scroll and no horizontal shell scroll.
- One-row horizontally scrollable tab rails with overflow fades.
- Active-tab reveal inside the rail.
- Center drop for stack insertion and edge drop for split insertion.
- Strip-priority reorder while the pointer stays in the source strip band.
- Action context inheritance: commands opened from a pane target that pane.
- Retained inactive-tab state through snapshots or mounted hidden bodies.
- Invalid-drop no-op behavior.

## Do Not Copy

- Do not copy the palette. qivxif uses the dark minimal tokens from
  [../design/visual-language.md](../design/visual-language.md).
- Do not copy Nostr-specific product tools.
- Do not copy relay settings, profile-only tools, or signing surfaces.
- Do not require a hidden app backend for local workspace behavior.

## qivxif Surfaces

qivxif tabs should cover these product surfaces:

- Setup.
- Welcome.
- Graph Node.
- Text Node.
- Graph Map.
- Feed.
- Publishing.
- History.
- Sync Status.
- Settings.
- Diagnostics.
- Admin.
- Profile.
- Media.

## Source Boundary

- qivxif uses SvelteKit components under `src/lib/components/workspace/`.
- The browser surface mirrors app shell, app header, split node, pane, pane head,
  tab strip, tab frame, tab stack, tile menu, new-tab button, `PaneDropLayer`,
  resize handle, and tab body.
- Domain reducers and effect adapters own durable state transitions.
- Components emit `WorkspaceCommand` and render controller snapshots.
