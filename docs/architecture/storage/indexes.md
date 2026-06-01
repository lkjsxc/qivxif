# Indexes

## Owned Indexes

- `user_names`: login name to `UserId`.
- `edges_by_from`: `(from_node, edge_id)` to empty marker.
- `edges_by_to`: `(to_node, edge_id)` to empty marker.
- `event_ids_by_actor`: `(actor_id, actor_seq)` to `EventId`.
- `event_ids_by_parent`: `(parent_event_id, child_event_id)` to empty marker.
- `event_ids_by_target_node`: `(node_id, event_id)` to empty marker.
- `event_ids_by_target_edge`: `(edge_id, event_id)` to empty marker.
- `event_ids_by_target_event`: `(target_event_id, event_id)` to empty marker.
- `feed_items_by_user`: `(user_id, event_id)` to empty marker.

## Rules

- Graph traversal does not scan all edges.
- Feed queries do not scan all posts.
- Index rebuilds are available through `qivxifctl`.
- Index keys sort lexicographically by packed tuple order.
- Primary write and index write happen in the same transaction.
- Tombstoned records remain in indexes unless the query explicitly filters them.
- Repair checks compare every index entry against its primary record.
- Sync pull scans cursor order, then filters candidate events by target node ACL.

## Maintenance

- User creation inserts `users` first, then `user_names`.
- Edge creation inserts `edges`, then both edge indexes.
- Event acceptance inserts `events_by_id`, then actor and target indexes.
- Feed item creation inserts `feed_items`, then per-user feed indexes.
- Tombstone events update primary records and keep indexes for history queries.
