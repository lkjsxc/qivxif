# Event Envelope

## Fields

- `event_id`
- `actor_id`
- `actor_seq`
- `parents`
- `scope`
- `kind`
- `target_node_ids`
- `target_edge_ids`
- `target_event_ids`
- `payload`
- `payload_hash`
- `created_at_client`
- `received_at_server`
- `auth_context`

## ID Rule

Event IDs are random and time-free. They never embed timestamps, counters,
actor sequence, host data, shard data, or sortable data.

## Time Rule

- `created_at_client` is display and diagnostics metadata.
- `received_at_server` is acceptance metadata.
- Neither timestamp is ID material.
- Neither timestamp is a correctness cursor.

## Payload Hash

- `payload_hash` is calculated over canonical payload bytes.
- The canonical format is defined by the payload owner before acceptance.
- Hash mismatch rejects the event before reducer application.

## Validation

- Event ID must match the ID contract.
- Actor sequence must be present and greater than zero.
- Kind must be known.
- Payload schema must match the kind.
- Duplicate target arrays are rejected when the kind requires unique targets.
- Parent order is preserved exactly as submitted.
