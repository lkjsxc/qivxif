# Event Graph

## Purpose

The event graph records causal ancestry and explicit semantic relations without
overloading either one.

## Causal Parents

- `parents` contains event IDs that the actor observed before writing.
- Parent references form a causal DAG.
- Parent references are not required to form a tree.
- Parent references are not semantic relations by themselves.

## Semantic Relations

Semantic relations are represented by edge events and indexes:

- `event.parent`
- `event.references`
- `node.parent`
- `node.contains`
- `node.ordered_child`
- `edge.supersedes`
- `edge.tombstones`
- `board.contains`
- `tile.contains_pane`
- `pane.views_node`

## Indexes

The store maintains relation indexes by event, node, and edge target so history,
tree projection, and inspection do not scan the whole log.

## Rule

Timestamps never determine graph correctness. Reducers use event IDs, actor
sequence rules, parent references, explicit relation edges, and deterministic
store acceptance order only where a tie-breaker is documented.
