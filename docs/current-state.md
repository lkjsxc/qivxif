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
- The product has no Leptos UI shell, no Dexie path, no fake protocol content,
  and no hidden app backend requirement.

## Evidence In The Repository

- The visible browser client is SvelteKit under `apps/qivxif-web/src/lib/components/`.
- No Cargo member or package dependency names Leptos.
- No JavaScript package dependency names Dexie.
- Tiled pane, tab stack, hidden inactive tab bodies, drop layer, and Svelte
  product surfaces are present.
- Browser storage now opens a dedicated SQLite WASM worker through
  `src/lib/storage/`, with OPFS as normal mode and memory as degraded mode.
- Active product source no longer calls `indexedDB.open`.
- The browser controller now opens concrete storage, HTTP, service worker, and
  sync effects through `src/lib/app/browser-ports.ts`.
- Rust crates already own graph, history, auth, sync, cache model, quality, and
  store logic that can be reused for WASM kernels or optional sync services.

## Open Migration Lanes

- Complete the dispatch migration so components receive typed command dispatch
  instead of the `actionsFor` migration adapter.
- Route effect modules from the typed `LocalStore` adapter to repository-family
  methods where it improves clarity.
- Refactor the browser controller toward typed `WorkspaceCommand`, pure reducer
  plans, and `AppPorts` that hide storage, HTTP, workers, service worker, sync,
  and WASM bridge details.
- Build initial WASM service modules after the storage seam is stable, starting
  with tile reducers, row codecs, canonical payload hashing, and sync planning.
- Server routes and redb storage remain optional sync and serving code. Product
  UI must not depend on direct server account flow as its local truth.
- Touch pointer dragging needs the full long-press and edge-split path, beyond
  the native drag coverage currently used by offline browser checks.

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

After the SQLite worker switch, host diagnostics passed:

- `cd apps/qivxif-web && npm run build`
- `cargo check --locked -p qivxif-server`
- `cd apps/qivxif-web && npm run check`
- `cargo run --locked -p qivxifctl -- quality check-lines`
- `cargo run --locked -p qivxifctl -- quality check-browser-storage`
- `cargo run --locked -p qivxifctl -- quality check-workspace`
- `cargo run --locked -p qivxifctl -- docs validate-topology`

Docker Compose verification is pending for the final gate after this batch.

## Audit Link

See [product/doc-impl-audit.md](product/doc-impl-audit.md) for row-by-row evidence.
