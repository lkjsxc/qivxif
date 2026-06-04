# Accepted Decisions

## Product

- qivxif is a browser-first Svelte workspace for graph knowledge work.
- `docs/` is the durable source of truth.
- The browser app opens directly into a compact header plus tiled tab shell.
- First-run owner setup lives inside the tile shell when optional sync service
  setup is enabled.
- Board composition is a qivxif-native graph surface.
- No retired local GUI shell canon remains.
- No backward compatibility is preserved during the reset.

## Architecture

- Svelte owns rendered product UI.
- Rust/WASM owns deterministic kernels, reducers, codecs, and typed host bridges.
- Browser SQLite WASM owns local durable storage through a worker.
- OPFS is the normal browser storage mode.
- Memory storage is allowed only as an explicit degraded mode with diagnostics.
- Product code reaches storage through typed repositories.
- Optional Axum routes may serve static assets and sync lanes behind ports.
- Optional redb storage may back a sync service but is not browser local truth.
- Durable data uses typed records, typed graph edges, and append-only events.
- The app shell and static assets use a service worker and Cache API.
- Client cache behavior is governed by a cache orchestrator.

## Client Implementation

- The browser shell uses SvelteKit with a Vite production build to `dist/`.
- Components emit `WorkspaceCommand`; the controller owns state transitions.
- Components never call raw storage, OPFS, IndexedDB, SQL, fetch, or workers directly.
- The first editor uses a textarea only when edits map into qivxif text events.
- CodeMirror may replace the widget after the durable text event model is stable.
- Rust reducers are the first durable authority for shared deterministic logic.
- Browser UI modules emit commands; reducers and effect adapters own event
  drafts, persistence, and sync.

## Protocol and Media

- The first durable HTTP wire format is JSON using `qivxif-api` DTOs when the
  optional sync service is present.
- Binary framing waits until HTTP sync behavior is proven by integration tests.
- HTTP push and pull are the always-available durable sync lane for the optional
  service.
- WebTransport is preferred when available and uses the same durable message types.
- Media chunking waits until upload thresholds and public serving behavior are documented.
