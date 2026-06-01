# Indexes

## Owned Indexes

- `edges_by_from`
- `edges_by_to`
- `ops_by_actor`
- `ops_by_node`
- `feed_items_by_user`
- `user_names`

## Rules

- Graph traversal does not scan all edges.
- Feed queries do not scan all posts.
- Index rebuilds are available through `qivxifctl`.
