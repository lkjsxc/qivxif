# Push Pull

## HTTP Lane

- `POST /api/sync/push` uploads durable events.
- `GET /api/sync/pull` returns events after a cursor.
- Batch limits protect server memory.
- Rejections are structured.

## Push Request

| Field | Type | Rule |
| --- | --- | --- |
| `client_id` | string | stable per browser profile |
| `actor_id` | `ActorId` | must match session authority |
| `events` | event envelope array | hard limit from server-info |
| `cursor_summary` | object | last applied and uploaded positions |

## Push Response

| Field | Type | Rule |
| --- | --- | --- |
| `accepted` | array | event id, server cursor, commit group |
| `rejected` | array | event id and structured error |
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
| `events` | event envelope array | ordered by server cursor |
| `server_cursor` | cursor | resume token for later pull |
| `has_more` | boolean | true when another pull is needed |

## Pull Visibility

- Pull requires an authenticated session.
- The store filters every returned event through server ACL.
- An event with target nodes is visible only when every target node is readable
  by the viewer.
- Events with no readable target are excluded instead of redacted.
- Hidden events do not advance the returned cursor for that viewer.
- The cursor never reveals event count or timestamp information for hidden records.

## Acceptance Rules

- The server validates auth for each event.
- Duplicate accepted events return the previous acceptance.
- Unknown event kinds use `schema.unknown_event_kind`.
- Mixed batches may contain both accepted and rejected entries.
- Cursor order is deterministic and independent of wall-clock time.
- `node.create` and `edge.create` envelopes update graph records.
- `text.insert`, `text.delete`, and `text.restore` envelopes update text projections.
- Text event payload bytes are canonical JSON for the text model from
  [../text/crdt.md](../text/crdt.md).

## Live Lane

WebTransport reliable streams carry the same durable message types after authentication.
