# Axum API

## Envelope

Every `/api` response uses [../schema/api-envelope.md](../schema/api-envelope.md).

## Routes

| Method | Path | Auth | CSRF | Body | Success payload | Side effect | Offline relation |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `GET` | `/health` | public | no | none | health status | none | app can show server reachability |
| `GET` | `/api/server-info` | public | no | none | capability list and limits | none | app learns feature flags |
| `POST` | `/api/auth/login` | public | no | login name, password | user, session summary, csrf token | creates session | requires server |
| `POST` | `/api/auth/logout` | session | yes | none | logout status | deletes session | queued logout is not accepted |
| `GET` | `/api/me` | session | no | none | current user and profile | none | stale local user may be shown as offline |
| `POST` | `/api/nodes` | session | yes | node create request | node record and operation acceptance | appends operation, writes node | local op queues first |
| `GET` | `/api/nodes/{node_id}` | viewer | no | none | node projection | none | IndexedDB may satisfy stale read |
| `GET` | `/api/nodes/{node_id}/history` | viewer | no | limit query | operation summaries | none | IndexedDB may show cached summaries |
| `POST` | `/api/edges` | session | yes | edge create request | edge record and operation acceptance | appends operation, writes edge and indexes | local op queues first |
| `GET` | `/api/nodes/{node_id}/edges` | viewer | no | direction and limit query | edge list | none | IndexedDB may satisfy stale read |
| `GET` | `/api/graph/neighborhood` | viewer | no | node, depth, limit query | bounded graph projection | none | IndexedDB may satisfy stale read |
| `POST` | `/api/sync/push` | session | yes | operation batch | accepted and rejected operation results | appends accepted operations | queued while offline |
| `GET` | `/api/sync/pull` | session | no | cursor, scope, limit query | operation batch and cursor | none | resumes after reconnect |
| `GET` | `/api/text/{node_id}` | viewer | no | none | text document projection | none | IndexedDB may satisfy stale read |
| `POST` | `/api/text/{node_id}/ops` | session | yes | text operation request | operation acceptance and text projection | appends text op | local op queues first |
| `GET` | `/api/feed/home` | session | no | cursor and limit query | feed items | none | cached feed window may render |
| `POST` | `/api/publish/{node_id}` | session | yes | publish request | publication state | appends publish op | queued pending server validation |
| `POST` | `/api/unpublish/{node_id}` | session | yes | unpublish request | publication state | appends unpublish op | queued pending server validation |

## Error Codes

- Auth failures use `auth.*` codes.
- Schema failures use `schema.*` codes.
- Operation failures use `operation.*` codes.
- Store failures use `store.*` codes.
- Sync failures use `sync.*` codes.
- Publishing conflicts use `publish.*` codes.

See [../schema/error-codes.md](../schema/error-codes.md).

## Graph Mutation Payloads

`POST /api/nodes` requires:

- `op_id`
- `actor_seq`
- `node_id`
- `kind`
- `visibility`
- `metadata_map`

The server supplies owner, actor, receive time, operation payload hash, and current commit group. Repeating the same `op_id` returns the prior acceptance.

`POST /api/edges` requires:

- `op_id`
- `actor_seq`
- `edge_id`
- `from_node`
- `to_node`
- `kind`
- `metadata_map`

The server requires write access on `from_node` and read access on `to_node`. Repeating the same `op_id` returns the prior acceptance.

## Graph Query Payloads

`GET /api/graph/neighborhood` requires query parameters:

- `node_id`
- `depth`, default `1`, maximum `3`
- `limit`, default `50`, maximum `100`

The response contains a bounded `GraphProjection`. The server checks ACL for each node before adding it to the projection. Edges are included only when both endpoint nodes are visible in the projection.

## Text Operation Payloads

`POST /api/text/{node_id}/ops` requires:

- `actor_seq`
- `operation`

`operation` is the ordered character-id text operation from [../text/crdt.md](../text/crdt.md). The server wraps it in the durable operation envelope, stores the text projection, and returns the operation acceptance.

The first browser editor may send `text.restore` for whole-text saves. Each restore operation uses a fresh operation id and a monotonic `first_seq` range for the actor so character ids remain unique.

## Browser Route Flush

The browser queue stores the exact JSON request used by each durable mutation route. A queued entry is accepted only when the route response envelope contains the matching operation acceptance. Route flush is not a separate durability model; the server route must append the operation log entry before success.

## History Payloads

`GET /api/nodes/{node_id}/history` returns operation summaries for viewers who can read the node. The route exposes ids, scope, kind, actor sequence, payload hash, targets, and receive time. It does not expose payload bytes.

## Handler Rules

- Handlers parse DTOs, extract auth context, call domain services, and wrap envelopes.
- Handlers do not open redb transactions directly.
- Handlers do not implement reducers.
- Request ID is present in logs and response envelopes.
