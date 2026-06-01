# Node Records

## Fields

- `id`
- `kind`
- `owner_user_id`
- `created_by`
- `created_at`
- `updated_at`
- `visibility`
- `acl_ref`
- `current_commit_group`
- `current_text_ref`
- `metadata_map`
- `tombstone`

## Rules

- Node IDs are typed.
- Known kinds are validated.
- Tombstones preserve history.
- Direct node creation routes receive `op_id`, `actor_seq`, `node_id`, `kind`, `visibility`, and `metadata_map`.
- The server assigns owner, actor, receive time, and operation indexes during acceptance.
