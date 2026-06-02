# lkjstr Parity

## Purpose

This file defines what "like lkjstr except colors" means for qivxif.

## Copy

- Copy the interaction grammar, not the source framework.
- Copy the compact header plus full-height recursive workspace shape.
- Copy N-way splits with resize handles, tab splitting, stacking, moving,
  closing, maximizing, and restoring.
- Copy pane-local vertical scroll and no horizontal shell scroll.
- Copy one-row horizontally scrollable tab rails with overflow fades.
- Copy active-tab reveal inside the rail.
- Copy center drop for stack insertion and edge drop for split insertion.
- Copy strip-priority reorder while the pointer stays in the source strip band.
- Copy action context inheritance: commands opened from a pane target that pane.
- Copy retained inactive-tab state through snapshots or mounted hidden bodies.
- Copy invalid-drop no-op behavior.

## Do Not Copy

- Do not copy the palette. qivxif uses the Zed-minimal dark tokens from
  [../design/visual-language.md](../design/visual-language.md).
- Do not copy Nostr-specific product tools.
- Do not copy relay settings, Mine npub, profile-only tools, or signing surfaces.
- Do not turn qivxif into a browser-only local app.

## qivxif Surfaces

qivxif tabs should cover these product surfaces:

- Setup
- Welcome
- Graph Node
- Text Node
- Board
- Feed
- Publishing
- History
- Sync Status
- Settings
- Diagnostics

## Source Boundary

- qivxif uses SvelteKit components under `src/lib/components/workspace/`.
- The browser surface mirrors: app shell, app header, split node, pane, pane head,
  tab strip, tab frame, tab stack, `PaneDropLayer`, and tab body.
- Domain reducers and effect adapters own durable state transitions.
- Components emit `WorkspaceCommand` and render controller snapshots.
