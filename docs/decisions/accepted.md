# Accepted Decisions

## Product

- qivxif is a browser-first Svelte workspace for graph knowledge work.
- `docs/` is the durable source of truth.
- The browser app opens directly into a compact header plus tiled tab shell.
- First-run owner setup lives inside the tile shell when optional sync service
  setup is enabled.
- Graph Map is the 2D graph projection surface.
- Resource orchestration is a core product differentiator.
- No retired local GUI shell canon remains.
- No backward compatibility is preserved during the reset.

## Architecture

- Svelte owns rendered product UI.
- Rust/WASM owns deterministic kernels, reducers, codecs, and typed host bridges.
- Browser SQLite WASM owns local durable storage through a worker.
- OPFS is the normal browser storage mode.
- Memory storage is allowed only as an explicit degraded mode with diagnostics.
- Product code reaches storage through typed repositories.
- Optional Axum routes may serve static assets, sync lanes, media, and public pages behind ports.
- Optional redb storage may back a sync service but is not browser local truth.
- Durable data uses typed records, typed graph edges, and append-only events.
- The app shell and static assets use a service worker and Cache API.
- Client cache, media retention, graph indexes, and background jobs are governed by the resource orchestrator.

## Client Implementation

- The browser shell uses SvelteKit with a Vite production build to `dist/`.
- Components emit `WorkspaceCommand`; the controller owns state transitions.
- Components never call raw storage, OPFS, IndexedDB, SQL, fetch, or workers directly.
- The editor widget is acceptable only when edits map into qivxif text events.
- CodeMirror may replace the widget after the durable text event model is stable.
- Rust reducers are the first durable authority for shared deterministic logic.
- Pure Rust reducers are shared with the browser behind TypeScript WASM service
  modules after fixture parity is proven for each moved reducer.
- Browser UI modules emit commands; reducers and effect adapters own event
  drafts, persistence, and sync.

## Protocol, Media, And Keys

- The first durable HTTP wire format is JSON using `qivxif-api` DTOs when the
  optional sync service is present.
- Binary framing waits until HTTP sync behavior is proven by integration tests.
- HTTP push and pull are the always-available durable sync lane for the optional service.
- WebTransport is preferred when available and uses the same durable message types.
- Media bytes use content-addressed chunks outside raw SQLite and redb value fields.
- Administrators can issue invites and scoped API tokens with audit evidence.
