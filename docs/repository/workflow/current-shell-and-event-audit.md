# Current Shell And Event Audit

## Evidence

- Graph, sync, and offline queue foundations are stable.
- `docker compose -f docker-compose.verify.yml run --rm verify` is the static gate.
- `docker compose run --rm api-test` and `docker compose run --rm offline-e2e` pass on the
  SvelteKit client build.
- Design system docs live under `docs/product/design/`.
- N-way layout is documented in `docs/product/tile-shell/layout-tree.md`.
- Drag resolver geometry is documented in `docs/product/tile-shell/drag-drop.md` and
  `docs/architecture/client/drag-resolver.md`.

## Retired DOM Shell

The prior minimal TypeScript plus direct DOM renderer under `apps/qivxif-web/src/ui/`
is retired. It is replaced by SvelteKit components under `src/lib/components/`.
Do not restore DOM-only shell modules as canon.

## Implementation State

- `qivxif-graph` uses N-way `TileTree::Split { axis, children, sizes }`.
- `AppController` owns workspace state, dispatches `WorkspaceCommand`, and notifies
  subscribers; effects implement IO and reducers stay pure in `domain/`.
- SvelteKit client ships under `apps/qivxif-web/src/lib/` with workspace shell, product
  surfaces, and Vite `dist/` output consumed by `qivxif-server`.
- IndexedDB stores include `local_workspace`, `dirty_events`, `accepted_events`,
  `tab_snapshots`, and `sync_cursors`.

## Residual Gaps

- Pointer rail drag for tab reorder is not wired in Svelte; offline E2E uses HTML5 drag
  for the equivalent assertion.
- Public blog HTML assertions remain simplified in `publish-flow.mjs`.
- CodeMirror, light theme, and feed virtualization remain out of scope for this slice.

## File Discipline

- Docs remain at or below 300 lines.
- Web, graph, and Svelte modules stay at or below 200 lines per file.
