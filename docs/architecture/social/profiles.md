# Profiles

## User Profile Node

Each durable user owns one `profile` node.

`StoredUser` contains:

- `id`
- `actor_id`
- `profile_node_id`
- `name`
- `password_hash`
- `roles`

Profile node metadata:

- `name`: login name at creation time.
- `profile_state`: `active`.

Profile nodes are public graph records owned by the user. Follow edges target profile nodes rather than raw user records.

## Creation

Admin bootstrap and user creation write the user record and profile node in one redb transaction.

The profile node id is returned in login and `/api/me` user summaries.
