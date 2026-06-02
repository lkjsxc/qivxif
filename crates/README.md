# Rust Crates

Workspace libraries for the qivxif server, graph reducers, storage, and quality gates.

## Crates

- [qivxif-api](qivxif-api/README.md): HTTP routes, DTO envelopes, setup and social APIs.
- [qivxif-auth](qivxif-auth/README.md): Sessions, CSRF, ACL checks, password hashing.
- [qivxif-cache-model](qivxif-cache-model/README.md): Cache orchestration types for the browser.
- [qivxif-core](qivxif-core/README.md): Shared IDs, time, visibility, and core errors.
- [qivxif-graph](qivxif-graph/README.md): Node and edge records, tile layout commands, graph reducers.
- [qivxif-history](qivxif-history/README.md): Event envelopes, commit groups, text CRDT reducers.
- [qivxif-quality](qivxif-quality/README.md): Repository topology, line limits, and wording gates.
- [qivxif-store-redb](qivxif-store-redb/README.md): redb tables, feeds, sync accept, publish.
- [qivxif-sync](qivxif-sync/README.md): Push and pull validation, sync queue types.

## Rules

- Durable business rules live here before browser mirrors.
- Each crate directory has one `README.md` table of contents.
- Source files stay at 200 lines or fewer.

## Related Docs

- [../docs/architecture/README.md](../docs/architecture/README.md)
- [../docs/repository/layout/workspace-layout.md](../docs/repository/layout/workspace-layout.md)
