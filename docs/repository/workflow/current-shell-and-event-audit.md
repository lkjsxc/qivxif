# Current Shell And Event Audit

## Evidence

- Working tree was clean before edits.
- Current branch is `main`.
- Remote is `https://github.com/lkjsxc/qivxif`.
- Baseline gate passed with `docker compose -f docker-compose.verify.yml run --rm verify`.
- The gate reported web build, web syntax, Rust format, clippy, tests, doctests,
  build, docs topology, line limits, wording, retired-canon, markers,
  workspace, route, and redb table checks as `ok`.

## Documentation State

- `docs/` is the owner canon and already has one `README.md` in each directory.
- Markdown files are below the 300-line limit.
- The active lane still describes a Rust-first Web super app reset, but it does
  not yet name the tiled shell parity and event-graph reset lane.
- The reading order still points to operation-log and operation-kind documents
  as the durable mutation contract.
- The ID owner doc says random bodies are 32 bytes, while implementation still
  generates UUID-width bodies.

## Implementation State

- `qivxif-core` owns typed IDs in `crates/qivxif-core/src/ids.rs`.
- `qivxif-history` owns operation envelopes and payload validation.
- `qivxif-graph` owns pure node, edge, and tile layout structures.
- `qivxif-store-redb` persists operation log data and projection tables.
- `qivxif-server` exposes setup, auth, graph, history, sync, text, tile layout,
  social, publishing, moderation, and static file routes.
- `qivxif-web` already has TypeScript actor and UI modules, but tab movement and
  split behavior are still command-heavy rather than parity-grade drag behavior.

## Mismatches

- Durable mutation language is still operation-first instead of event-first.
- `OperationId` is exposed with prefix `op`; the requested durable ID target is
  random, time-free `EventId` values with prefix `evt`.
- Event-to-event, event-to-edge, and edge-event relation indexes are not yet a
  documented first-class model.
- Tree projection over accepted relation edges is not yet explicit.
- Tile shell docs do not yet spell out horizontally scrollable tab rails,
  strip-priority reorder, edge-split drops, retained inactive tab state, and
  independent visible tab instances.
- Browser UI modules are split, but the hard surface boundary between render
  modules, reducers, persistence, and sync effects needs a stronger contract.

## Next Canon Changes

- Set the active lane to tiled shell parity plus event-graph reset.
- Add a tile-shell parity document for the interaction grammar copied from the
  reference app while keeping qivxif colors and product surfaces.
- Add a replaceable client surface boundary.
- Add event architecture documents and update schema links.
- Replace the ID contract with 64 lowercase hex characters from 32 random bytes.
