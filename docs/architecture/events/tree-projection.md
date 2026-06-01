# Tree Projection

## Purpose

A tree is a projection over accepted relation edges. The durable model remains a
general graph.

## Inputs

- Root node ID.
- Accepted edge records.
- Accepted tombstone events or tombstoned edge records.
- Optional owner-specific relation kinds.

## Tombstones

The normal current-state projection ignores tombstoned relation edges.

History-oriented projections may surface tombstoned relation errors, but current
product views must be able to represent moves by tombstoning the old edge and
creating a new relation edge.

## Ordering

Tree children sort by:

1. Explicit `position_key` when present.
2. Explicit `ordinal` when present.
3. Deterministic store acceptance order only as a documented tie-breaker.

Wall-clock time is never an ordering source.

## Errors

Tree projection surfaces these errors instead of hiding them:

- Missing child node.
- Cycle.
- Duplicate active parent when the tree requires one parent.
- Tombstoned relation in a history-oriented projection.

## Graph Links

Cross-links remain valid graph edges even when they do not belong to a tree
projection.
