# Current Shell And Event Audit

## Evidence

- Working tree was clean before the UI reset lane started.
- Current branch is `main`.
- Remote is `https://github.com/lkjsxc/qivxif`.
- Baseline gate passed on 2026-06-02 with
  `docker compose -f docker-compose.verify.yml run --rm verify`.
- The gate reported web build, web syntax, Rust format, clippy, tests, doctests,
  build, docs topology, line limits, wording, retired-canon, markers,
  workspace, route, and redb table checks as `ok`.

## Documentation State

- `docs/` is the owner canon and already has one `README.md` in each directory.
- Markdown files are below the 300-line limit.
- Event architecture docs own the durable mutation contract.
- The ID owner doc and core implementation agree on 32 random bytes rendered as
  64 lowercase hex characters.
- Design system docs do not exist yet under `docs/product/design/`.
- N-way tile layout and drag-resolver geometry are not documented yet.
- Surface boundary docs still describe the actor pattern instead of controller
  and ports.

## Implementation State

- `qivxif-core` owns typed IDs in `crates/qivxif-core/src/ids.rs`.
- `qivxif-history` owns event envelopes, event kinds, payload hashes, and text
  event reducers.
- `qivxif-graph` owns pure node, edge, and binary tile layout structures.
- `qivxif-store-redb` persists append-only events plus node, edge, parent, and
  target indexes.
- `qivxif-server` exposes setup, auth, graph, history, sync, text, tile layout,
  social, publishing, moderation, and static file routes.
- `qivxif-web` uses `src/actors/` for shell bootstrap, actions, sync, and tile
  commands.
- `qivxif-web` renders a Catppuccin-like dark shell with binary splits and no
  resize handles.

## Gaps Before UI Reset

| Area | Current | Target |
| --- | --- | --- |
| Visual design | Catppuccin-like, 6–8px radii | Zed-minimal dark, 1–2px radii, dense chrome |
| Layout model | Binary `first`/`second` splits | N-way splits with `sizes[]` and resize handles |
| Drag geometry | Full-tile zones, 22%, 420ms long-press | Chrome/body split, strip priority, 28%, 250ms |
| Tab retention | Single active body rendered | Hidden-mount tab stack plus snapshots |
| Browser arch | `src/actors/` action table | `app/controller.ts` plus domain, effects, ui |
| Product surfaces | Bare DOM panels | Polished Zed-density tab surfaces |

## Next Implementation Changes

- Add design system docs, then N-way layout and drag-resolver docs.
- Migrate Rust and browser tile trees to N-way splits with resize reducers.
- Refactor browser code to controller, ports, and effects.
- Rebuild shell UI with Zed-minimal tokens and lkjstr interaction parity.
- Expand IndexedDB inspection for accepted, dirty, and rejected events.
