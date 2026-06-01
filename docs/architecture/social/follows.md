# Follows

## Route Contract

`POST /api/social/follow` accepts:

- `op_id`
- `actor_seq`
- `edge_id`
- `target_profile_node_id`

The server creates a `follows` edge from the signed-in user's profile node to the target profile node.

`POST /api/social/unfollow` accepts:

- `op_id`
- `actor_seq`
- `edge_id`

The server tombstones the active follow edge. History keeps the edge record.

## Feed Fan-Out

When a short post is accepted, the store writes feed markers for:

- the author user.
- each user with an active `follows` edge to the author's profile node.

Unfollow removes existing home-feed markers for the target author's feed items from the follower user index.

## Rules

- Following yourself is rejected.
- The target must be a readable `profile` node.
- Repeating the same follow operation returns the prior acceptance.
- Repeating a follow edge that is already active returns the existing active edge.
- Unfollow of a missing or already tombstoned edge is idempotent for the supplied operation id only.
