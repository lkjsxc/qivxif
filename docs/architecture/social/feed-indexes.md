# Feed Indexes

## Feed Item

`FeedItem` is stored in `feed_items` keyed by the creating operation id.

Fields:

- `operation_id`
- `post_node_id`
- `author_user_id`
- `author_name`
- `body`
- `visibility`
- `created_at`
- `reply_to`

## User Index

`feed_items_by_user` stores `(user_id, operation_id)` to an empty marker.

Accepted short posts create an author feed marker. Follow fan-out uses the same marker shape for follower user ids; absent follower edges means only the author marker is written.

## Query Contract

`GET /api/feed/home` requires a session and returns:

- `items`
- `cursor`
- `has_more`

Rules:

- Query uses `feed_items_by_user`; it must not scan all post nodes.
- Result order is newest accepted item first.
- `limit` is clamped to the documented server maximum.
- Private items are returned only to their author.
- Public, unlisted, and shared items remain subject to ACL filters before leaving the store boundary.
