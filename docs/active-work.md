# Active Work

## Current Lane

Svelte product UI, Rust/WASM pure kernels, worker-owned SQLite, compact tiled
workspace interaction, and honest local-first diagnostics.

## Active Targets

- Keep docs canonical before behavior changes.
- Keep every rendered product surface in Svelte.
- Move deterministic reducers and codecs behind typed WASM services.
- Move browser durable state behind a SQLite worker repository boundary.
- Preserve OPFS as the normal local storage mode and memory as an explicit
  degraded mode.
- Keep Leptos, Dexie, and direct component storage access out of product code.
- Make the tab header compact: rail first, plus button beside the rail, split
  and close actions in the three-dot menu.
- Make New Tab a real chooser tab that converts in place.
- Keep Docker Compose as the acceptance boundary.

## Stop Condition

The repo is coherent when all of these are true:

- `/` renders the Svelte shell immediately with the documented styling.
- Empty local storage opens a usable Welcome or Setup tab with storage
  diagnostics.
- Tabs can split, stack, move, reorder, close, restore, and resize locally.
- Each visible tab has independent state.
- Hidden tab stack retains inactive tab scroll and drafts.
- Drag geometry matches [product/tile-shell/drag-drop.md](product/tile-shell/drag-drop.md).
- Every durable local mutation is represented in the event queue repository.
- UI emits `WorkspaceCommand` only; controller owns state transitions.
- Components call typed actions, never raw storage or transport APIs.
- `npm run build` produces Vite `dist/` assets consumed by verify and smoke services.
- Docker Compose verification passes.
- Another agent can read [README.md](README.md), run the Compose verification
  path, and continue from committed slices without hidden context.
