# Active Work

## Current Lane

SvelteKit client migration, Zed-minimal chrome, lkjstr tile interaction parity, and
polished product surfaces.

## Active Targets

- Keep docs canonical before behavior changes.
- Migrate the browser client from the retired DOM shell to SvelteKit plus Vite.
- Apply Zed-minimal design tokens across shell and product surfaces.
- Port lkjstr drag geometry: dedicated drop layer, dual pointer and native drag.
- Mount inactive tabs in a hidden tab stack with independent pane state.
- Rebuild feed and social surfaces with timeline card patterns.
- Keep Docker Compose as the acceptance boundary.

## Stop Condition

The repo is coherent when all of these are true:

- `/` renders the Svelte shell immediately with Zed-minimal styling.
- Empty store opens Setup as a tab.
- Tabs can split, stack, move, reorder, close, restore, and resize locally.
- Each visible tab has independent state.
- Hidden tab stack retains inactive tab scroll and drafts.
- Drag geometry matches [product/tile-shell/drag-drop.md](product/tile-shell/drag-drop.md).
- Every durable mutation is represented as an event with a random ID.
- UI emits `WorkspaceCommand` only; controller owns state transitions.
- `npm run build` produces Vite `dist/` assets consumed by verify and smoke services.
- Docker Compose verification passes.
- Another agent can read [README.md](README.md), run the Compose verification
  script, and continue from committed slices without hidden context.
