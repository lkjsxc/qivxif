# Indexes

## Owned Indexes

- `user_names`: login name to `UserId`.
- `edges_by_from`: `(from_node, edge_id)` to empty marker.
- `edges_by_to`: `(to_node, edge_id)` to empty marker.
- `ops_by_actor`: `(actor_id, actor_seq)` to `OperationId`.
- `ops_by_node`: `(node_id, operation_id)` to empty marker.
- `feed_items_by_user`: `(user_id, operation_id)` to empty marker.

## Rules

- Graph traversal does not scan all edges.
- Feed queries do not scan all posts.
- Index rebuilds are available through `qivxifctl`.
- Index keys sort lexicographically by packed tuple order.
- Primary write and index write happen in the same transaction.
- Tombstoned records remain in indexes unless the query explicitly filters them.
- Repair checks compare every index entry against its primary record.

## Maintenance

- User creation inserts `users` first, then `user_names`.
- Edge creation inserts `edges`, then both edge indexes.
- Operation acceptance inserts `ops`, then actor and node operation indexes.
- Feed item creation inserts `feed_items`, then per-user feed indexes.
- Tombstone operations update primary records and keep indexes for history queries.
