# Active Work

## Current Lane

Svelte product UI, Rust/WASM pure kernels, worker-owned SQLite, lkjstr shell
parity, fast N-ary tile layout, usable editor, Graph Map, media foundation,
admin keys, profiles, and resource orchestration.

## Active Targets

- Keep docs canonical before behavior changes.
- Keep every rendered product surface in Svelte.
- Keep worker-owned SQLite as the only active browser storage path.
- Complete the browser shell migration from the `actionsFor` adapter to typed
  `WorkspaceCommand`, pure reducer plans, and `AppPorts`.
- Match lkjstr shell grammar for header, panes, tab rails, drag, split, stack,
  maximize, restore, resize, and invalid-drop no-ops.
- Make the tile tree N-ary with no persistence during pointermove.
- Build the standard editor contract: input, selection, undo/redo, IME, search,
  Markdown preview, local persistence, and reload restore.
- Replace retired surface code with Graph Map over real nodes and edges.
- Add media metadata, chunks, upload resume, range serving, and ACL.
- Add invite codes, scoped API tokens, revocation, CLI, routes, and audit.
- Add profile view and edit surfaces backed by profile nodes.
- Expand cache planning into resource orchestration with diagnostics.
- Move deterministic reducers and codecs behind typed WASM services.
- Keep Docker Compose as the acceptance boundary.

## Stop Condition

The repo is coherent when all of these are true:

- `/` renders the Svelte shell immediately with the documented styling.
- Empty local storage opens a usable Welcome or Setup tab with storage diagnostics.
- Tabs can split, stack, move, reorder, close, restore, and resize locally.
- Each visible tab has independent state.
- Hidden tab stack retains inactive tab scroll and drafts.
- Drag geometry matches [product/tile-shell/drag-drop.md](product/tile-shell/drag-drop.md).
- Every durable local mutation is represented in the event queue repository.
- UI emits `WorkspaceCommand` only; controller owns state transitions.
- Components receive dispatchers, never effect tables, raw storage, or transport APIs.
- Graph Map shows real graph records and dimension toggles.
- The editor can be used for real writing and survives refresh.
- Media, profile, admin key, and resource diagnostics use real records.
- `npm run build` produces Vite `dist/` assets consumed by verify and smoke services.
- Docker Compose verification passes.
- Another agent can read [README.md](README.md), run the Compose verification
  path, and continue from committed slices without hidden context.
