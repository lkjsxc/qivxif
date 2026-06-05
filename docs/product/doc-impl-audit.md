# Product Documentation Implementation Audit

## Purpose

This audit keeps the product contract, current evidence, and open work aligned.
Rows change only when the owner doc and implementation change together.

## Status Rows

| Contract | Owner doc | Evidence | State |
| --- | --- | --- | --- |
| Svelte frontend remains product UI | [architecture/client/rendering.md](../architecture/client/rendering.md) | `WorkspaceRoot.svelte` mounts the shell | Done |
| Leptos UI shell retired | [architecture/client/wasm-boundary.md](../architecture/client/wasm-boundary.md) | No Leptos Cargo member or package dependency | Done |
| Direct DOM shell removed | [repository/workflow/current-shell-and-event-audit.md](../repository/workflow/current-shell-and-event-audit.md) | `src/ui` tree absent | Done |
| SQLite worker owns durable browser state | [architecture/client/sqlite-worker.md](../architecture/client/sqlite-worker.md) | `src/lib/storage/sqlite.worker.ts` owns SQLite WASM; controller opens `current-store.ts` | Done |
| IndexedDB active path removed | [architecture/client/storage-migration.md](../architecture/client/storage-migration.md) | `effects/indexed-db.ts` deleted; active source has no `indexedDB.open` | Done |
| Dexie absent from product path | [architecture/client/sqlite-worker.md](../architecture/client/sqlite-worker.md) | `check-browser-storage` rejects Dexie in web app source | Done |
| Components emit typed commands only | [architecture/client/surface-boundary.md](../architecture/client/surface-boundary.md) | Controller still adapts through `actionsFor` | Open |
| AppPorts hide storage and network details | [architecture/client/surface-boundary.md](../architecture/client/surface-boundary.md) | Controller imports `browser-ports.ts`; `actionsFor` remains the migration adapter | In review |
| Svelte to WASM service boundary | [architecture/client/wasm-boundary.md](../architecture/client/wasm-boundary.md) | Rust kernels exist; generated bridge services still open | Open |
| New Tab is a real chooser tab | [tile-shell/tiled-tabs.md](tile-shell/tiled-tabs.md) | `NewTab.svelte` is inserted by the plus action | Done |
| Selected New Tab kind converts same tab id | [tile-shell/tiled-tabs.md](tile-shell/tiled-tabs.md) | `replaceTabInLayout` preserves `pane_node_id` | Done |
| Active tab alone exposes close | [tile-shell/tiled-tabs.md](tile-shell/tiled-tabs.md) | `TabFrame.svelte` renders close only inside active branch | Done |
| Split and close tile actions live in three-dot menu | [tile-shell/tiled-tabs.md](tile-shell/tiled-tabs.md) | `TileMenu.svelte` owns split, stack, maximize, and close actions | Done |
| Pane body edge split excludes header chrome | [tile-shell/drag-drop.md](tile-shell/drag-drop.md) | Resolver measures body rect and header band | Done |
| Touch long-press tab drag | [tile-shell/drag-drop.md](tile-shell/drag-drop.md) | Native drag path covers current checks | Open |
| Settings shows SQLite storage diagnostics | [tile-shell/settings-pane.md](tile-shell/settings-pane.md) | Settings and Diagnostics render worker mode, reason, quota, page count, inventory, queue, cache, and last error | Done |
| Product surfaces avoid fake content | [design/tab-surfaces.md](design/tab-surfaces.md) | Empty states use real local data and absence copy; scan remains required after storage switch | In review |
| Docker Compose gates pass | [../operations/verification/compose-pipeline.md](../operations/verification/compose-pipeline.md) | verify, offline-e2e, server-smoke, and api-test passed | Done |

## Update Rule

When a row changes, update this file, [../current-state.md](../current-state.md),
and the owner doc in the same coherent change.
