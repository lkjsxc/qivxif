# Axum and redb

## Axum

- Axum routes HTTP handlers.
- `State` extracts shared app state.
- `.with_state(state)` provides router state before serving.

## redb

- redb is a pure Rust embedded key-value database.
- It provides ACID transactions, MVCC readers, and crash-safe copy-on-write storage.
- Tables are declared with typed `TableDefinition` values.

## Implication

Axum owns request handling. redb stays behind the store crate boundary.
