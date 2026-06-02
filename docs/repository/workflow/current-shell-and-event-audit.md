# Current Shell And Event Audit

## Evidence

- Graph, sync, and offline queue foundations are stable.
- `docker compose -f docker-compose.verify.yml run --rm verify` is the static gate.
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
- Svelte migration is in flight: target tree is documented in
  `docs/architecture/client/module-layout.md`.
- IndexedDB stores include `local_workspace`, `dirty_events`, `accepted_events`,
  `tab_snapshots`, and `sync_cursors`.

## Residual Gaps

- Svelte workspace components and product surfaces must reach parity with tile-shell docs.
- Per-tab draft switching assertion in offline E2E may need selector updates after Svelte.
- Public blog HTML assertions remain simplified in `publish-flow.mjs`.

## File Discipline

- Docs remain at or below 300 lines.
- Web, graph, and Svelte modules stay at or below 200 lines per file.
