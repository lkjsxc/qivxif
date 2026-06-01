# Eviction

## Order

1. Reconstructable media previews.
2. Old feed windows.
3. Cold graph-neighbor projections.
4. Old text snapshots reconstructable from durable events.
5. Old public node bodies that are not pinned.

## Never Evict

- Dirty local events.
- Local-only nodes.
- Pinned nodes.
- Current tile layout.
- Cache manifest.
- Eviction journal.
