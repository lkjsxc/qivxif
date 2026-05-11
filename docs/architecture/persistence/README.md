# Persistence Architecture

Use this subtree for durable state facts.

## Current Implementation

- `WorldStore` opens a redb database at `world.redb`.
- The active redb tables are `meta` and `sections`.
- `meta` stores `WorldMeta` under key `world`.
- `sections` stores chunk edit overlays.
- `ArchiveStore` supports local object_store manifest write/read/list.

## Child Index

- [hot-state.md](hot-state.md): local authoritative database.
- [schema-contracts.md](schema-contracts.md): durable table ownership.
- [object-archives.md](object-archives.md): implemented archive manifest boundary.
- [backup-restore.md](backup-restore.md): current persistence verification facts.
