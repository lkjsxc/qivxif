# Current State

## Target

The frontend target is Svelte plus WASM plus SQLite.

- Svelte owns all product UI and route rendering.
- Rust/WASM owns pure reducers, codecs, storage row transforms, runtime planners,
  and typed browser-host bridge functions.
- A browser worker owns SQLite WASM. OPFS is the normal durable path and memory
  mode is an explicit degraded path with diagnostics.
- Product modules call typed repositories. They do not open raw storage or
  network transports directly.
- The product has no Leptos UI shell, no Dexie path, no invented protocol content,
  and no hidden app backend requirement.
- Graph Map replaces the retired canvas term in product canon.

## Repository Reconciliation

The root README now matches this Svelte plus WASM plus SQLite target. Any stale
non-browser shell description outside `docs/` is not canon. `docs/` remains the
authority for behavior and architecture.

## Source Inventory

| Contract area | Owner docs | Main implementation files |
| --- | --- | --- |
| Svelte shell | `product/tile-shell/` | `apps/qivxif-web/src/lib/components/workspace/` |
| Controller boundary | `architecture/client/surface-boundary.md` | `src/lib/app/controller.ts`, `src/lib/domain/workspace-command.ts` |
| Tile reducer | `product/tile-shell/layout-tree.md` | `src/lib/domain/tile-*.ts`, `crates/qivxif-graph/src/tile_*` |
| Drag resolver | `product/tile-shell/drag-drop.md` | `src/lib/domain/drop-resolver.ts`, workspace pane components |
| SQLite repositories | `architecture/client/sqlite-worker.md` | `src/lib/storage/` |
| Editor | `product/editor/` | `src/lib/components/surfaces/EditorTab.svelte`, text effects |
| Graph records | `architecture/graph/` | `crates/qivxif-graph/`, graph routes and store modules |
| Graph Map | `product/graph-map/` | `GraphMapTab.svelte`, `graph-map-actions.ts`, and `graph-map-view.ts` use real graph records |
| Media | `product/media/`, `architecture/media/` | browser imports files as OPFS chunks with metadata and graph attachment edges; server routes are open |
| Admin keys | `product/admin/`, `architecture/server/key-issuance.md` | CLI and route extension is open |
| Profiles | `product/profiles/`, `architecture/social/profiles.md` | user creation stores profile ids; edit surfaces are open |
| Resource orchestration | `product/resource-orchestration/` | `resource-planner.ts` explains protection and actions in Settings and Diagnostics |

## Evidence In The Repository

- The visible browser client is SvelteKit under `apps/qivxif-web/src/lib/components/`.
- No Cargo member or package dependency names Leptos.
- No JavaScript package dependency names Dexie.
- Tiled pane, tab stack, hidden inactive tab bodies, drop layer, and Svelte
  product surfaces are present.
- Browser storage opens a dedicated SQLite WASM worker through `src/lib/storage/`,
  with OPFS as normal mode and memory as degraded mode.
- Active product source no longer calls `indexedDB.open`.
- The browser controller opens concrete storage, HTTP, service worker, and sync
  effects through `src/lib/app/browser-ports.ts`.
- Rust crates own graph, history, auth, sync, cache model, quality, and store logic.

## Open Migration Lanes

- Complete the dispatch migration so components receive typed command dispatch
  instead of the `actionsFor` migration adapter.
- Complete lkjstr shell parity for drag, touch drag, tab rail behavior, and tile menus.
- Make the N-ary tile reducer the only tile model and enforce performance budgets.
- Finish the standard editor reducer path: character-id range events, remote merge,
  conflict fixtures, and deeper accessibility checks.
- Extend Graph Map with richer dimension controls, manual edge forms, layout cancellation, and saved view state.
- Build server media routes, upload resume, range serving, thumbnail jobs, and ACL checks.
- Build admin invite codes, scoped API tokens, revocation, CLI, routes, and audit.
- Build profile edit surfaces and avatar media edges.
- Add resource plan executors, leases, persistent journal writes, and media/cache mutation jobs.
- Build initial WASM service modules after the storage seam is stable.

## Completed Facts

- Svelte is the only rendered product UI path in this repository.
- The old direct DOM renderer is gone from `apps/qivxif-web/src/ui/`.
- Leptos and Dexie are absent from active dependency manifests.
- `qivxifctl quality check-browser-storage` rejects Dexie, active
  `indexedDB.open`, raw SQL outside storage worker files, component raw storage,
  and direct worker messages outside the storage client.
- The New Tab command opens a real chooser tab and selected kinds convert that pane in place.
- The active tab alone exposes the close control.
- Split, stack, maximize, and tile close actions live in the three-dot menu.
- Settings and Diagnostics render SQLite worker mode, reason, usage, quota,
  page count, inventory, queue counts, cache bytes, and last storage error.
- Docker Compose remains the final acceptance boundary.

## Latest Verification

After the SQLite worker, AppPorts, and offline test updates, these gates passed:

- `docker compose -f docker-compose.yml config`.
- `docker compose --progress quiet -f docker-compose.yml build qivxif verify offline-e2e server-smoke api-test`.
- `docker compose --progress quiet -f docker-compose.yml run --rm verify`.
- `docker compose --progress quiet -f docker-compose.yml run --rm offline-e2e`.
- `docker compose --progress quiet -f docker-compose.yml run --rm server-smoke`.
- `docker compose --progress quiet -f docker-compose.yml run --rm api-test`.
- `cargo run --locked -p qivxifctl -- quality check-lines`.
- `cargo run --locked -p qivxifctl -- quality check-placeholders`.
- `cargo run --locked -p qivxifctl -- quality check-browser-storage`.
- `cargo run --locked -p qivxifctl -- docs validate-topology`.

## Audit Link

See [product/doc-impl-audit.md](product/doc-impl-audit.md) for row-by-row evidence.
