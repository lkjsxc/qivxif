# Repository Gates

## Checks

- `cargo fmt -- --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo test --doc --workspace`
- `cargo build --workspace`
- `qivxifctl docs validate-topology`
- `qivxifctl quality check-lines`
- `qivxifctl quality check-wording`
- `qivxifctl quality check-retired-canon`
- `qivxifctl quality check-public-names`
- `qivxifctl quality check-placeholders`
- `qivxifctl quality check-workspace`
- `qivxifctl quality check-routes`
- `qivxifctl quality check-redb-tables`

## Route Gate

`check-routes` compares the route table in
`docs/architecture/server/axum-api.md` with Axum `.route(...)` declarations
under `apps/qivxif-server/src/routes`.

The gate fails when:

- a documented route is not mounted.
- a mounted route is not documented.

## redb Table Gate

`check-redb-tables` compares the table list in
`docs/architecture/storage/redb-layout.md` with `tables::ALL` in
`crates/qivxif-store-redb/src/tables.rs`.

The gate fails when a table name is missing from either side or when the
documented table order differs from the store registry.
