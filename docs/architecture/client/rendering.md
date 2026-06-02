# Rendering

## Rules

- Virtualize long feeds when lists exceed roughly 200 items (follow-up slice).
- Avoid full workspace rerenders on pointer move; update drop layer via rAF.
- Use stable pane and tab IDs in keys and `data-pane-id`.
- Keep text inside its container at supported viewports.

## SvelteKit Shell

- The browser shell uses SvelteKit with a static adapter output to `dist/`.
- `WorkspaceRoot.svelte` subscribes to `AppController` snapshots.
- Components never mutate IndexedDB; they dispatch `WorkspaceCommand`.
- The first visible surface is header plus tile grid, not a marketing page.
- No permanent sidebar or dashboard lives outside tiles.

## Styling

- Global tokens import from `src/lib/styles/tokens.css` in `+layout.svelte`.
- Shell and surface styles are plain CSS files, not CSS-in-JS.
