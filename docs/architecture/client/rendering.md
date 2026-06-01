# Rendering

## Rules

- Virtualize long feeds and search results.
- Avoid full app rerenders on cursor movement.
- Use requestAnimationFrame for drag layout updates.
- Use stable IDs for panes and tabs.
- Keep text inside its container at supported viewports.

## First Browser Shell

- The first shell uses minimal TypeScript modules and DOM rendering.
- A larger UI framework waits until actor-driven DOM rendering becomes the limiting factor.
- Components send actor messages instead of mutating IndexedDB directly.
- The first visible surface is the workspace, not a marketing page.
