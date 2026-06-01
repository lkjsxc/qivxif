# Accepted Decisions

## Product

- qivxif is a Rust-first server/client Web super app.
- `docs/` is the durable source of truth.
- The browser workspace is tiled and tabbed.
- `kjxlkj` is the graph composition workspace.
- The server supports multiple users in one instance.
- No native desktop shell canon remains.
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
