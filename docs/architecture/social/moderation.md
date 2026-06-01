# Moderation

## Edge Contract

Moderation state is graph data.

`mutes` edge:

- from signed-in user's `profile` node.
- to muted user's `profile` node.
- hides target-author feed items from the viewer.
- does not prevent public reads outside feeds.

`blocks` edge:

- from signed-in user's `profile` node.
- to blocked user's `profile` node.
- hides target-author feed items from the blocker.
- hides blocker-author feed items from the target viewer.
- prevents replies from either side to the other side's posts.

## Routes

`POST /api/social/mute` and `POST /api/social/block` accept:

- `event_id`
- `actor_seq`
- `edge_id`
- `target_profile_node_id`

`POST /api/social/unmute` and `POST /api/social/unblock` accept:

- `event_id`
- `actor_seq`
- `edge_id`

The server appends the matching moderation event and writes or tombstones the
edge in the same transaction.

## Query Rules

- Feed queries use `feed_items_by_user` first.
- Feed results are filtered for ACL and moderation before response creation.
- Mute is one-way.
- Block is two-way for feed visibility and reply creation.
- Removing a mute or block makes existing feed markers visible again when markers still exist.
- Feed rebuild may restore markers removed by other graph actions.

## Idempotency

- Repeating an accepted `event_id` returns the prior acceptance.
- Repeating an active moderation edge appends the event but does not create
  another active edge.
- Clearing a missing or already tombstoned edge rejects unless the clearing
  event was already accepted.
