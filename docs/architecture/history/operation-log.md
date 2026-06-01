# Operation Log

## Envelope Fields

- `op_id`
- `actor_id`
- `actor_seq`
- `parents`
- `scope`
- `kind`
- `target_node_ids`
- `payload`
- `payload_hash`
- `created_at_client`
- `received_at_server`
- `auth_context`

## Rules

- Every durable mutation appends an operation.
- Duplicate upload is idempotent.
- Timestamps are not correctness cursors.
