# Visual Language

## Direction

qivxif uses a **Zed-minimal dark** visual language. The workspace should feel
tool-neutral, dense, and calm. Color is functional, not decorative.

## Principles

- Near-black backgrounds with one or two raised surface steps.
- Sharp UI radii: 1–2px for panels, tabs, and controls.
- Dense chrome: header height 40–44px, pane header 36–40px.
- Muted accent for focus and active states, not saturated brand color.
- Long IDs, diagnostics, and queue entries use monospace.
- Body text uses system UI sans-serif.
- No gradients, glass effects, or heavy shadows except command palette overlay.
- No horizontal shell scroll. Pane bodies scroll vertically only.

## Layout Grammar

Copy lkjstr workspace grammar:

- Header plus recursive tile grid fills the viewport.
- One root tile tree.
- Pane header owns tile menu, tab rail, and new-tab control.
- Tab bodies fill remaining pane height.
- Long content wraps inside the visible pane.

Do not copy lkjstr palette or Nostr-specific chrome.

## Density

- Tab rail: single row, horizontal scroll, fade edges on overflow.
- Forms: compact labels above fields, 8px vertical rhythm inside sections.
- Lists and tables: tight row height, clear separators, no card sprawl.
- Status chips in header: small, inline, readable at a glance.

## Focus And Motion

- Focus rings use a single accent outline, 2px, visible on keyboard focus.
- Drag previews use a flat accent tint, never animated glow.
- Resize handles are 4px hit targets with 1px visible line on hover.
- Transitions are optional and short; never block interaction.

## Forbidden Patterns

- Pill tabs with large corner radius.
- Purple or pastel "editor theme" palettes on shell chrome.
- Centered marketing-style landing layouts inside product tabs.
- Placeholder lorem or fake data on the main path.
- External palette names in public docs, source, tests, or styles.
