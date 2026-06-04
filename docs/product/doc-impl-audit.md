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
| SQLite worker owns durable browser state | [architecture/client/sqlite-worker.md](../architecture/client/sqlite-worker.md) | `indexed-db.ts` still present | Open |
| Dexie absent from product path | [architecture/client/sqlite-worker.md](../architecture/client/sqlite-worker.md) | No Dexie package dependency | Done |
| Svelte to WASM service boundary | [architecture/client/wasm-boundary.md](../architecture/client/wasm-boundary.md) | Rust kernels exist; generated bridge services still open | Open |
| New Tab is a real chooser tab | [tile-shell/tiled-tabs.md](tile-shell/tiled-tabs.md) | Current UI uses header chooser overlay | Open |
| Selected New Tab kind converts same tab id | [tile-shell/tiled-tabs.md](tile-shell/tiled-tabs.md) | Current action opens a new pane | Open |
| Active tab alone exposes close | [tile-shell/tiled-tabs.md](tile-shell/tiled-tabs.md) | Tab frame has no active close control yet | Open |
| Split and close tile actions live in three-dot menu | [tile-shell/tiled-tabs.md](tile-shell/tiled-tabs.md) | Direct header buttons still present | Open |
| Pane body edge split excludes header chrome | [tile-shell/drag-drop.md](tile-shell/drag-drop.md) | Resolver measures body rect and header band | Done |
| Touch long-press tab drag | [tile-shell/drag-drop.md](tile-shell/drag-drop.md) | Native drag path covers current checks | Open |
| Settings shows typed storage diagnostics | [tile-shell/settings-pane.md](tile-shell/settings-pane.md) | Settings shows session and pane counts only | Open |
| Product surfaces avoid fake content | [design/tab-surfaces.md](design/tab-surfaces.md) | Empty states use real local data and absence copy | In review |

## Update Rule

When a row changes, update this file, [../current-state.md](../current-state.md),
and the owner doc in the same coherent change.
