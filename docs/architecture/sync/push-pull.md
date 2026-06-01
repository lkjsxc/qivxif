# Push Pull

## HTTP Lane

- `POST /api/sync/push` uploads durable operations.
- `GET /api/sync/pull` returns operations after a cursor.
- Batch limits protect server memory.
- Rejections are structured.

## Push Request

| Field | Type | Rule |
| --- | --- | --- |
| `client_id` | string | stable per browser profile |
| `actor_id` | `ActorId` | must match session authority |
| `operations` | operation envelope array | hard limit from server-info |
| `cursor_summary` | object | last applied and uploaded positions |

## Push Response

| Field | Type | Rule |
| --- | --- | --- |
| `accepted` | array | operation id, server cursor, commit group |
| `rejected` | array | operation id and structured error |
| `server_cursor` | cursor | highest durable server position in response |
| `capabilities` | string array | active sync capabilities |

## Pull Request

| Field | Type | Rule |
| --- | --- | --- |
| `cursor` | cursor | not a timestamp |
| `limit` | integer | clamped by server |
| `scope` | string | graph, text, feed, or all |

## Pull Response

| Field | Type | Rule |
| --- | --- | --- |
| `operations` | operation envelope array | ordered by server cursor |
| `server_cursor` | cursor | resume token for later pull |
| `has_more` | boolean | true when another pull is needed |

## Acceptance Rules

- The server validates auth for each operation.
- Duplicate accepted operations return the previous acceptance.
- Unknown operation kinds use `schema.unknown_operation_kind`.
- Mixed batches may contain both accepted and rejected entries.
- Cursor order is deterministic and independent of wall-clock time.
- `node.create` and `edge.create` envelopes update graph records.
- `text.insert`, `text.delete`, and `text.restore` envelopes update text projections.
- Text operation payload bytes are canonical JSON for the text operation model from [../text/crdt.md](../text/crdt.md).

## Live Lane

WebTransport reliable streams carry the same durable message types after authentication.
