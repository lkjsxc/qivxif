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
- Rust crates already own graph, history, auth, sync, cache model, quality, and
  store logic that can be reused for WASM kernels or optional sync services.

## Open Migration Lanes

- `apps/qivxif-web/src/lib/effects/indexed-db.ts` still owns browser local data
  and must move behind the SQLite worker repository boundary.
- Server routes and redb storage still exist as optional sync and serving code.
  Product UI must not depend on direct server account flow as its local truth.
- Pointer tab dragging needs the full long-press and edge-split path, beyond the
  native drag coverage currently used by offline browser checks.
- Settings and Diagnostics now show local store mode, reason, usage, quota, and
  inventory; SQLite-backed protected and prunable cache bytes remain open.

## Completed Facts

- Svelte is the only rendered product UI path in this repository.
- The old direct DOM renderer is gone from `apps/qivxif-web/src/ui/`.
- Leptos and Dexie are absent from active dependency manifests.
- `qivxifctl quality check-browser-storage` rejects Dexie in web app source.
- The New Tab command opens a real chooser tab and selected kinds convert that pane in place.
- The active tab alone exposes the close control.
- Split, stack, maximize, and tile close actions live in the three-dot menu.
- Docker Compose remains the final acceptance boundary.

## Audit Link

See [product/doc-impl-audit.md](product/doc-impl-audit.md) for row-by-row evidence.
