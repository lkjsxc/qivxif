# Accepted Decisions

## Product

- qivxif is a Rust-first server/client Web super app.
- `docs/` is the durable source of truth.
- The browser app opens directly into a compact header plus tiled tab shell.
- First-run owner setup lives inside the tile shell, not on a separate page.
- Board composition is a qivxif-native graph surface.
- The server supports multiple users in one instance.
- No retired local GUI shell canon remains.
- No backward compatibility is preserved during the reset.

## Architecture

- Axum owns HTTP API, public routes, auth routes, and static app serving.
- redb owns embedded server storage.
- WebTransport owns preferred live sync.
- Durable sync uses reliable streams or HTTP.
- Datagrams are only for ephemeral hints.
- The durable data model is typed KV plus typed graph.
- Nodes and edges are first-class records.
- All durable mutations append operations.
- Text nodes use CRDT-backed operation history.
- Client offline data lives in IndexedDB.
- The app shell and static assets use a service worker and Cache API.
- Client cache behavior is governed by a cache orchestrator.

## Client Implementation

- The first browser shell uses minimal TypeScript modules and DOM rendering.
- The first editor uses a textarea only when edits map into qivxif text operations.
- CodeMirror may replace the widget after the durable text operation model is stable.
- Server-side Rust reducers are the first durable authority.
- Browser code mirrors only the DTO checks needed for local storage until reducer sharing is documented.

## Protocol and Media

- The first durable HTTP wire format is JSON using `qivxif-api` DTOs.
- Binary framing waits until HTTP sync behavior is proven by integration tests.
- HTTP push and pull are the always-available durable sync lane.
- WebTransport is preferred when available and uses the same durable message types.
- Media chunking waits until upload thresholds and public serving behavior are documented.
