# redb Layout

## File

The default server database file is `data/qivxif.redb`.

## Tables

- `meta`
- `users`
- `user_names`
- `sessions`
- `nodes`
- `edges`
- `edges_by_from`
- `edges_by_to`
- `ops`
- `ops_by_actor`
- `ops_by_node`
- `commit_groups`
- `blobs`
- `blob_chunks`
- `text_docs`
- `text_snapshots`
- `feed_items`
- `feed_items_by_user`
- `auth_tokens`
- `sync_cursors`
- `server_jobs`

## Rule

Store values are compact typed bytes. JSON belongs at HTTP boundaries.
