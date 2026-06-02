# Reference Patterns

## Purpose

This file records what qivxif borrows from mature products. External names stay in
docs only; CSS and components use qivxif tokens.

## Tile Shell Grammar

Borrow from the compact tiled workspace pattern:

- Fixed viewport shell with header plus recursive split grid.
- Pane-local tab rails with horizontal scroll and overflow fades.
- Center drop for stack insertion; edge drop for split insertion.
- Hidden tab stack keeps inactive bodies mounted.
- Invalid drops are no-ops.

## Tool Chrome Density

Borrow from IDE-minimal dark tools:

- Near-black backgrounds with one or two raised surface steps.
- 1–2px radii on panels, tabs, and controls.
- Header height 40–44px; pane chrome 36–40px.
- Muted accent for focus and active states.
- Monospace for IDs and diagnostics.

## Timeline Surfaces

Borrow from modern social clients for feed tabs only:

- Compose bar pinned at top of the feed pane.
- Post cards with author row, body, and inline actions.
- Avatar column or initials chip at fixed width.
- Relative timestamps in muted text.
- Empty states explain follow graph requirements.

## Do Not Copy

- Foreign palettes or brand accent colors.
- Product-specific tools outside qivxif surfaces.
- Card sprawl or marketing landing layouts inside tabs.
- Heavy shadows, glass effects, or pill tabs on shell chrome.

## Implementation Mapping

| Pattern | Owner doc | Code home |
| --- | --- | --- |
| Tile drag | [../tile-shell/drag-drop.md](../tile-shell/drag-drop.md) | `lib/workspace/`, `PaneDropLayer.svelte` |
| Tokens | [tokens.md](tokens.md) | `src/lib/styles/tokens.css` |
| Feed cards | [surfaces-feed.md](surfaces-feed.md) | `src/lib/components/surfaces/FeedTab.svelte` |
| Editor | [surfaces-editor.md](surfaces-editor.md) | `src/lib/components/surfaces/EditorTab.svelte` |
