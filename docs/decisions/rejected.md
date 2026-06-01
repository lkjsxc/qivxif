# Rejected Decisions

## Product

- Native desktop shell as primary product.
- File tree as primary data structure.
- Placeholder features.
- Plugin marketplace before core product stability.
- Social engagement ranking as default.

## Architecture

- Opaque giant JSON blob for graph state.
- Durable event delivery through unreliable datagrams.
- Mock modules.
- Direct component writes to IndexedDB.
- Auth only in the client.
- Feed queries by scanning all posts.
- Text history by full-copy-per-edit storage.
- External database requirement for core behavior.
- redb table access outside the store crate.
