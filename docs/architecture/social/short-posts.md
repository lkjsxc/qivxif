# Short Posts

## Record Shape

The first short-post slice stores each post as one `short_post` node.

Required node metadata:

- `body`: plain text post body.
- `author_name`: login name at acceptance time.
- `social_state`: `posted`.
- `posted_at`: server receive time.

Optional node metadata:

- `reply_to`: target `short_post` node id.

The body is plain text. Markdown preview is not part of this slice.

## Route

`POST /api/social/short-posts` accepts:

- `op_id`
- `actor_seq`
- `node_id`
- `body`
- `visibility`
- `reply_to`

The server supplies owner, actor, timestamps, feed item, and operation envelope.

## Rules

- Empty bodies are rejected.
- Bodies over 512 Unicode scalar values are rejected.
- `visibility` must be `private`, `shared`, `unlisted`, or `public`.
- `reply_to`, when present, must target a readable `short_post`.
- Acceptance appends `social.short_post_create`.
- Acceptance writes the node and feed item in the same redb transaction.
- Duplicate `op_id` returns the prior acceptance.
- Duplicate actor sequence for another operation is rejected.
