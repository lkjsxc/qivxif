# Schema Contracts

## Status

- Status: implemented for two redb tables.
- Owner: `crates/qivxif-storage::tables`.

## Active Tables

| Table | Key type | Value type | Current contents |
| --- | --- | --- | --- |
| `meta` | `&str` | `&[u8]` | key `world` stores postcard `WorldMeta` |
| `sections` | `&str` | `&[u8]` | chunk overlay keys store postcard `Vec<BlockCell>` |

## Files And Keys

- Database file: `world.redb`.
- Metadata key: `world`.
- Overlay key shape: `section/{chunk_x}/{chunk_z}`.
- Negative chunk coordinates keep their signed decimal form.

## Bootstrap

- `database::open` creates the root directory.
- `database::open` creates or opens `world.redb`.
- `init_tables` opens `meta` and `sections` in a write transaction.
- Hot-path reads and writes assume active tables already exist.

## Commit Boundary

- `put_chunk_overlay` writes one complete chunk overlay per transaction.
- The transaction sets `Durability::Immediate` before commit.
- A committed transaction is the boundary observed by restart probes.

## Not Implemented

- `profiles` table.
- `bases` table.
- `skills` table.
- `market_orders` table.
- `mail` table.
- True section keys with `SectionCoord { x, y, z }`.
