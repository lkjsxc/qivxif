# Feed Indexes

## Feed Item

`FeedItem` is stored in `feed_items` keyed by the creating event id.

Fields:

- `event_id`
- `post_node_id`
- `author_user_id`
- `author_name`
- `body`
- `visibility`
- `created_at`
- `reply_to`

## User Index

`feed_items_by_user` stores `(user_id, event_id)` to an empty marker.

Accepted short posts create an author feed marker. Follow fan-out uses the same marker shape for follower user ids; absent follower edges means only the author marker is written.

## Rebuild

`qivxifctl feeds rebuild` reconstructs `feed_items_by_user` from:

- all `feed_items`.
- each item author.
- active `follows` edges to the author's profile node.

The rebuild does not rewrite `feed_items`, post nodes, or event records.

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
