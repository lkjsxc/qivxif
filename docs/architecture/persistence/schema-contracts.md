# Schema Contracts

## Tables

- `meta`: world and schema epochs.
- `sections`: edited section payloads.
- `profiles`: player state.
- `bases`: base and claim records.
- `skills`: progression state.
- `market_orders`: regional listing and settlement records.
- `mail`: durable delivery records when mail exists.

## Encoding

- Hot-state records use compact explicit schemas.
- `postcard` is acceptable for compact hot records when the owner crate controls
  the schema.
- `rkyv` belongs to read-mostly archives and caches, not mutable hot truth.

## Rule

Each table has one owner crate and one owner doc.
