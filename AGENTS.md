# Agent Instructions

## Work Order

1. Read `docs/README.md` before changing behavior.
2. Update the owner doc before implementation.
3. Keep parent `README.md` indexes current.
4. Verify through Docker Compose when possible.
5. Commit small coherent batches.

## Product Direction

- Svelte owns all rendered product UI.
- Rust and WASM own pure kernels, reducers, codecs, runtime planning, and typed browser-host bridges.
- SQLite WASM in a worker owns durable browser state; product code calls typed repositories.
- Do not add or restore a Leptos product UI shell.
- Do not make Svelte components open SQLite, OPFS, IndexedDB, raw SQL, or network transports directly.

## Hard Limits

- Markdown files under `docs/`: `<=300` lines.
- Authored source files under `apps/`, `crates/`, and `scripts/`: `<=200` lines.
- Do not minify or remove useful names to fit limits.

## Wording

- Use `schema_contract`, `layout_contract`, or `build_contract` for durable formats.
- Avoid stale product-line labels and old-path aliases.
- Delete retired contracts instead of preserving parallel meanings.

## Protected Canon

`docs/` is authoritative. `tmp/` is research input only.
