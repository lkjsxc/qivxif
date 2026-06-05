# Profiles

## User Profile Node

Each durable user owns one `profile` node.

`StoredUser` contains:

- `id`.
- `actor_id`.
- `profile_node_id`.
- `name`.
- `password_hash`.
- `roles`.

Profile node metadata at creation:

- `name`: login name at creation time.
- `display_name`: initial display name.
- `profile_state`: `active`.

Profile nodes are public graph records owned by the user unless later profile
settings change field visibility. Follow edges target profile nodes rather than
raw user records.

## Creation

Admin bootstrap, user creation, and invite acceptance write the user record and
profile node in one redb transaction.

The profile node id is returned in login and `/api/me` user summaries.

## Editable Metadata

- display name.
- avatar media asset id.
- bio.
- links.
- visibility.

## Safety

Profile routes never return password hashes, session ids, CSRF hashes, token
hashes, or token secrets.
