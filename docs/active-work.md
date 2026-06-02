# Active Work

## Current Lane

Zed-minimal UI redesign plus N-way tile layout plus controller refactor.

## Active Targets

- Keep docs canonical before behavior changes.
- Apply Zed-minimal design tokens across the browser shell.
- Migrate tile layout to N-way splits with resize handles.
- Refactor browser code to controller, ports, domain, effects, and ui layers.
- Port lkjstr drag geometry: chrome and body regions, strip priority, shared
  resolver.
- Mount inactive tabs in a hidden tab stack with independent pane state.
- Expand IndexedDB inspection for accepted, dirty, and rejected events.
- Keep Docker Compose as the acceptance boundary.

## Stop Condition

The repo is coherent when all of these are true:

- `/` renders the shell immediately with Zed-minimal styling.
- Empty store opens Setup as a tab.
- Tabs can split, stack, move, reorder, close, restore, and resize locally.
- Each visible tab has independent state.
- Hidden tab stack retains inactive tab scroll and drafts.
- Drag geometry matches [product/tile-shell/drag-drop.md](product/tile-shell/drag-drop.md).
- Every durable mutation is represented as an event with a random ID.
- UI emits `WorkspaceCommand` only; controller owns state transitions.
- Docker Compose verification passes.
- Another agent can read [README.md](README.md), run the Compose verification
  script, and continue from committed slices without hidden context.
