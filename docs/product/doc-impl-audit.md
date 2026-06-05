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
| Components emit typed commands only | [architecture/client/surface-boundary.md](../architecture/client/surface-boundary.md) | Controller still adapts through `actionsFor` | Open |
| AppPorts hide storage and network details | [architecture/client/surface-boundary.md](../architecture/client/surface-boundary.md) | Controller imports `browser-ports.ts`; `actionsFor` remains migration adapter | In review |
| Svelte to WASM service boundary | [architecture/client/wasm-boundary.md](../architecture/client/wasm-boundary.md) | Rust kernels exist; generated bridge services still open | Open |
| lkjstr shell parity | [tile-shell/lkjstr-parity.md](tile-shell/lkjstr-parity.md) | Components exist; parity gaps remain in drag and touch paths | Open |
| N-ary tile layout contract | [tile-shell/layout-tree.md](tile-shell/layout-tree.md) | Smart insertion now appends siblings on matching axes; final type-shape migration remains open | In review |
| Tile performance budgets | [tile-shell/performance.md](tile-shell/performance.md) | Fixture and pointer budget gates are open | Open |
| New Tab is a real chooser tab | [tile-shell/tiled-tabs.md](tile-shell/tiled-tabs.md) | `NewTab.svelte` is inserted by the plus action | Done |
| Selected New Tab kind converts same tab id | [tile-shell/tiled-tabs.md](tile-shell/tiled-tabs.md) | `replaceTabInLayout` preserves `pane_node_id` | Done |
| Active tab alone exposes close | [tile-shell/tiled-tabs.md](tile-shell/tiled-tabs.md) | `TabFrame.svelte` renders close only inside active branch | Done |
| Pane body edge split excludes header chrome | [tile-shell/drag-drop.md](tile-shell/drag-drop.md) | Resolver measures body rect and header band | Done |
| Touch long-press tab drag | [tile-shell/drag-drop.md](tile-shell/drag-drop.md) | Native drag path covers current checks | Open |
| Standard editor | [editor/standard-editor.md](editor/standard-editor.md) | Editor has search, status, counts, preview, IME-aware draft updates, and local undo/redo buttons; CRDT range mapping remains open | In review |
| Graph Map replaces retired surface term | [graph-map/README.md](graph-map/README.md) | Source uses `graph_map`, `graph_map_item`, `placed_on_graph_map`, and `GraphMapTab.svelte` | Done |
| Graph dimensions | [../architecture/graph/dimensions.md](../architecture/graph/dimensions.md) | Graph Map has edge-kind toggles; richer dimensions remain open | In review |
| Automatic relationship suggestions | [../architecture/graph/auto-linking.md](../architecture/graph/auto-linking.md) | Suggestion records and UI are open | Open |
| Media foundation | [media/README.md](media/README.md) | Browser imports files into OPFS chunks, writes `media_asset` nodes, metadata, chunk rows, and attachment edges; server routes remain open | In review |
| Resource orchestration | [resource-orchestration/README.md](resource-orchestration/README.md) | Browser has pure resource planner, resource stores, and Settings/Diagnostics explanations; executors remain open | In review |
| Admin invites and keys | [admin/keys.md](admin/keys.md) | CLI can issue, list, revoke invites and keys, stores hashed secrets, and records audit rows; HTTP routes remain open | In review |
| Profile edit surfaces | [profiles/README.md](profiles/README.md) | User creation stores profile node id; UI editing is open | Open |
| Settings shows SQLite storage diagnostics | [tile-shell/settings-pane.md](tile-shell/settings-pane.md) | Settings and Diagnostics render worker mode, reason, quota, inventory, queue, cache, and last error | Done |
| Product surfaces avoid invented content | [design/tab-surfaces.md](design/tab-surfaces.md) | Empty states use real local data and absence copy; scan remains required after storage switch | In review |
| Docker Compose gates pass | [../operations/verification/compose-pipeline.md](../operations/verification/compose-pipeline.md) | verify, offline-e2e, server-smoke, and api-test passed | Done |

## Update Rule

When a row changes, update this file, [../current-state.md](../current-state.md),
and the owner doc in the same coherent change.
