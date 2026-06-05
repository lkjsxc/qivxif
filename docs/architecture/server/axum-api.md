# Axum API

## Envelope

Every `/api` response uses [../schema/api-envelope.md](../schema/api-envelope.md).

## Routes

| Method | Path | Auth | CSRF | Body | Success payload | Side effect | Offline relation |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `GET` | `/health` | public | no | none | health status | none | app can show server reachability |
| `GET` | `/api/setup` | public | no | none | setup status | none | app decides first-run tab |
| `POST` | `/api/setup/owner` | public | no | name and password | owner user, csrf token, next actor sequence | creates first owner and session | requires empty store |
| `GET` | `/api/server-info` | public | no | none | capability list and limits | none | app learns feature flags |
| `POST` | `/api/auth/login` | public | no | login name, password | user, session summary, csrf token, next actor sequence | creates session | requires server |
| `POST` | `/api/auth/logout` | session | yes | none | logout status | deletes session | queued logout is not accepted |
| `GET` | `/api/me` | session | no | none | current user and profile | none | stale local user may be shown as offline |
| `POST` | `/api/nodes` | session | yes | node create request | node record and event acceptance | appends event, writes node | local event queues first |
| `GET` | `/api/nodes/{node_id}` | viewer | no | none | node projection | none | local repository may satisfy stale read |
| `GET` | `/api/nodes/{node_id}/history` | viewer | no | limit query | event summaries | none | local repository may show cached summaries |
| `POST` | `/api/edges` | session | yes | edge create request | edge record and event acceptance | appends event, writes edge and indexes | local event queues first |
| `GET` | `/api/nodes/{node_id}/edges` | viewer | no | direction and limit query | edge list | none | local repository may satisfy stale read |
| `GET` | `/api/graph/neighborhood` | viewer | no | node, depth, limit query | bounded graph projection | none | local repository may satisfy stale read |
| `POST` | `/api/tile-layout` | session | yes | layout set request | layout node and event acceptance | appends layout event, writes layout metadata | local event queues first |
| `POST` | `/api/sync/push` | session | yes | event batch | accepted and rejected event results | appends accepted events | queued while offline |
| `GET` | `/api/sync/pull` | session | no | cursor, scope, limit query | event batch and cursor | none | resumes after reconnect |
| `GET` | `/api/text/{node_id}` | viewer | no | none | text document projection | none | local repository may satisfy stale read |
| `POST` | `/api/text/{node_id}/events` | session | yes | text event request | event acceptance and text projection | appends text event | local event queues first |
| `GET` | `/api/feed/home` | session | no | cursor and limit query | feed items | none | cached feed window may render |
| `POST` | `/api/social/short-posts` | session | yes | short post create request | post node, feed item, event acceptance | appends event, writes node and feed index | local event queues first |
| `POST` | `/api/social/follow` | session | yes | follow request | follow edge and event acceptance | appends event and writes edge | local event queues first |
| `POST` | `/api/social/unfollow` | session | yes | unfollow request | tombstoned follow edge and event acceptance | appends event, tombstones edge and removes feed markers | local event queues first |
| `POST` | `/api/social/mute` | session | yes | moderation request | mute edge and event acceptance | appends event and writes edge | local event queues first |
| `POST` | `/api/social/unmute` | session | yes | moderation clear request | tombstoned mute edge and event acceptance | appends event and tombstones edge | local event queues first |
| `POST` | `/api/social/block` | session | yes | moderation request | block edge and event acceptance | appends event and writes edge | local event queues first |
| `POST` | `/api/social/unblock` | session | yes | moderation clear request | tombstoned block edge and event acceptance | appends event and tombstones edge | local event queues first |
| `POST` | `/api/publish/{node_id}` | session | yes | publish request | publication state | appends publish event | queued pending server validation |
| `POST` | `/api/unpublish/{node_id}` | session | yes | unpublish request | publication state | appends unpublish event | queued pending server validation |
| `POST` | `/api/media/uploads` | session or token | yes for cookie | upload metadata | upload session | creates media upload session | local upload session persists |
| `GET` | `/api/media/assets/{asset_id}/content` | viewer | no | range query | media bytes | none | local media cache may satisfy visible content |
| `POST` | `/api/admin/invites` | admin | yes | invite request | invite secret once | stores invite hash and audit | server required |
| `POST` | `/api/admin/keys` | admin | yes | key request | token secret once | stores token hash and audit | server required |
| `POST` | `/api/invites/accept` | public | no | invite secret and account fields | user summary and profile | creates user and profile | server required |
| `GET` | `/@{author}/{slug}` | public | no | none | rendered blog post | none | cached public page may render |

## Error Codes

- Auth failures use `auth.*` codes.
- Schema failures use `schema.*` codes.
- Event failures use `event.*` codes.
- Store failures use `store.*` codes.
- Sync failures use `sync.*` codes.
- Publishing conflicts use `publish.*` codes.

See [../schema/error-codes.md](../schema/error-codes.md).

## Setup Payloads

`GET /api/setup` returns:

- `required`
- `owner_creation_open`

`POST /api/setup/owner` requires:

- `name`
- `password`

The server rejects owner creation when any durable user exists. A successful
response creates a normal session and returns the CSRF token used by later
authenticated mutations.

## Graph Mutation Payloads

`POST /api/nodes` requires:

- `event_id`
- `actor_seq`
- `node_id`
- `kind`
- `visibility`
- `metadata_map`

The server supplies owner, actor, receive time, event payload hash, and current
commit group. Repeating the same `event_id` returns the prior acceptance.

`POST /api/edges` requires:

- `event_id`
- `actor_seq`
- `edge_id`
- `from_node`
- `to_node`
- `kind`
- `metadata_map`

The server requires write access on `from_node` and read access on `to_node`.
Repeating the same `event_id` returns the prior acceptance.

## Tile Layout Payloads

`POST /api/tile-layout` requires:

- `event_id`
- `actor_seq`
- `layout_node_id`
- `layout`

The target node must be a `tile_layout` node owned by the actor or writable
by an admin. The server stores the canonical layout JSON in node metadata and
appends `tile.layout_set`. Repeating the same `event_id` returns the prior
acceptance.

## Graph Query Payloads

`GET /api/graph/neighborhood` requires query parameters:

- `node_id`
- `depth`, default `1`, maximum `3`
- `limit`, default `50`, maximum `100`

The response contains a bounded `GraphProjection`. The server checks ACL for each node before adding it to the projection. Edges are included only when both endpoint nodes are visible in the projection.

## Text Event Payloads

`POST /api/text/{node_id}/events` requires:

- `actor_seq`
- `event`

`event` carries the ordered character-id text edit from
[../text/crdt.md](../text/crdt.md). The server wraps it in the durable event
envelope, stores the text projection, and returns the event acceptance.

The first browser editor may send `text.restore` for whole-text saves. Each
restore event uses a fresh event id and a monotonic `first_seq` range for the
actor so character ids remain unique.

## Publishing Payloads

`POST /api/publish/{node_id}` requires:

- `event_id`
- `actor_seq`
- `slug`
- `summary`

The target must be a `blog_post` node with `body_node_id` metadata. The server
checks write access, slug uniqueness, appends `publish.post`, and makes the
post public.

`POST /api/unpublish/{node_id}` requires:

- `event_id`
- `actor_seq`
- `reason`

The server appends `publish.unpublish`, removes public access, and leaves
authorized history intact.

## Browser Route Flush

The browser queue stores the exact JSON request used by each durable mutation
route. A queued entry is accepted only when the route response envelope contains
the matching event acceptance. Route flush is not a separate durability model;
the server route must append the event log entry before success.

## History Payloads

`GET /api/nodes/{node_id}/history` returns event summaries for viewers who can
read the node. The route exposes ids, scope, kind, actor sequence, payload hash,
targets, and receive time. It does not expose payload bytes.

## Handler Rules

- Handlers parse DTOs, extract auth context, call domain services, and wrap envelopes.
- Handlers do not open redb transactions directly.
- Handlers do not implement reducers.
- Request ID is present in logs and response envelopes.
