# Current Shell And Event Audit

## Evidence

- Working tree was clean before edits.
- Current branch is `main`.
- Remote is `https://github.com/lkjsxc/qivxif`.
- Baseline gate passed with `docker compose -f docker-compose.verify.yml run --rm verify`.
- The gate reported web build, web syntax, Rust format, clippy, tests, doctests,
  build, docs topology, line limits, wording, retired-canon, markers,
  workspace, route, and redb table checks as `ok`.
- The event reset slice now passes local web build, cargo check, and workspace
  Rust tests.

## Documentation State

- `docs/` is the owner canon and already has one `README.md` in each directory.
- Markdown files are below the 300-line limit.
- The active lane names tiled shell parity plus event-graph reset.
- Event architecture docs own the durable mutation contract.
- The ID owner doc and core implementation agree on 32 random bytes rendered as
  64 lowercase hex characters.

## Implementation State

- `qivxif-core` owns typed IDs in `crates/qivxif-core/src/ids.rs`.
- `qivxif-history` owns event envelopes, event kinds, payload hashes, and text
  event reducers.
- `qivxif-graph` owns pure node, edge, and tile layout structures.
- `qivxif-store-redb` persists append-only events plus node, edge, parent, and
  target indexes.
- `qivxif-server` exposes setup, auth, graph, history, sync, text, tile layout,
  social, publishing, moderation, and static file routes.
- `qivxif-web` already has TypeScript actor and UI modules, but tab movement and
  split behavior are still command-heavy rather than parity-grade drag behavior.

## Mismatches

- Event terms now match across docs, Rust, API DTOs, sync queues, and browser
  local event queues.
- Tree projection is documented, but the Rust projection over relation events
  still needs a dedicated implementation slice.
- Tile shell docs spell out the target grammar, but the browser drag resolver
  and retained inactive-tab snapshots still need implementation work.
- Browser UI modules are split, and the hard surface boundary is documented;
  controller/reducer separation still needs a broader implementation pass.

## Next Implementation Changes

- Implement tree projection over relation events.
- Strengthen tile layout reducers around independent visible tab instances.
- Add parity-grade tab movement and edge split drag behavior.
- Expand browser persistence from a local event queue into accepted, dirty, and
  rejected event stores with tab snapshots.
