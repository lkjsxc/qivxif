# qivxif-store-redb

Embedded redb storage: graph tables, event log, feeds, publish, sync accept, repair.

## Source

- [src/lib.rs](src/lib.rs): store facade.
- [src/store.rs](src/store.rs): transaction boundary.
- [src/tables.rs](src/tables.rs): table definitions.
- [src/event_log.rs](src/event_log.rs): append and query events.
- [src/feed.rs](src/feed.rs): feed indexes and queries.

## Related Docs

- [../../docs/architecture/storage/README.md](../../docs/architecture/storage/README.md)
