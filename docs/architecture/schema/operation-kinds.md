# Operation Kinds

Every durable mutation uses one of these operation kinds. Unknown kinds are rejected before reducer application.

## Proof Slice Operations

| Kind | Auth | Targets | Payload | Reducer | Idempotency | Conflict | Offline | Server reject |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `user.bootstrap_admin` | administer | user | name, password hash, profile node | auth user insert | `op_id` plus actor seq | fails if admin exists | no | yes |
| `auth.login_session_created` | public | session | user, session, csrf hash | session insert | session id | fails on bad credentials | no | yes |
| `node.create` | write | node | kind, visibility, metadata | graph node insert | node id | duplicate returns prior acceptance | yes | yes |
| `node.update_metadata` | write | node | field patches | per-field register | `op_id` | deterministic tie break | yes | yes |
| `node.tombstone` | write | node | reason code | set tombstone | `op_id` | already tombstoned is no-op | yes | yes |
| `edge.create` | link | edge, nodes | from, to, kind, metadata | graph edge insert | edge id | duplicate returns prior acceptance | yes | yes |
| `edge.tombstone` | link | edge | reason code | set tombstone | `op_id` | already tombstoned is no-op | yes | yes |
| `text.create_doc` | write | node, text doc | text doc id | text doc insert | text doc id | duplicate returns prior acceptance | yes | yes |
| `text.insert` | write | text doc | position id, text | text CRDT insert | op id plus actor seq | deterministic CRDT merge | yes | yes |
| `text.delete` | write | text doc | range ids | text CRDT delete | op id plus actor seq | missing range rejects or waits | yes | yes |
| `text.restore` | write | text doc | snapshot ref | append restore text op | op id | creates new state, never erases | yes | yes |
| `workspace.layout_set` | write | workspace layout | tile tree delta | workspace reducer | op id | deterministic last accepted action | yes | yes |
| `sync.cursor_advance` | write | cursor | cursor id, position | cursor update | cursor id plus position | cannot move backward | no | yes |
| `publish.post` | publish | blog post | slug, summary, public time | publication reducer | op id | slug conflict rejects | queued | yes |
| `publish.unpublish` | publish | blog post | reason code | publication reducer | op id | already private is no-op | queued | yes |
| `social.short_post_create` | write | short post | body, visibility, optional reply target | graph plus feed reducer | node id | duplicate returns prior acceptance | yes | yes |
| `social.follow` | write | profiles | target profile | edge create | edge id | duplicate active edge is no-op | yes | yes |
| `social.unfollow` | write | follow edge | edge id | edge tombstone | op id | missing active edge rejects unless op was already accepted | yes | yes |

## Workspace Payload Contract

`workspace.layout_set` payload:

- `layout_node_id`: `workspace_layout` node being changed.
- `layout`: complete tile tree snapshot.
- `maximized_pane_id`: optional pane node ID inside the tile tree.

The reducer replaces the previous tile tree for the layout node after auth and
schema validation. Restore uses another `workspace.layout_set` operation.

## Board Placement Contract

Board placement uses graph operations:

- `node.create` creates a `board_item` placement record.
- `edge.create` with `placed_on_board` links board item to board.
- `edge.create` with `contains` links board item to displayed node.
- New movement appends another `board_item` with a higher `placement_seq`.

Clients choose the highest accepted or dirty `placement_seq` per board and item.

## Publishing Payload Contract

`publish.post` payload:

- `node_id`: `blog_post` node.
- `slug`: requested public slug.
- `summary`: public summary.
- `author_name`: server-owned route handle.

The server rejects slug conflicts for the same author. Acceptance makes the
post public and keeps the body text history unchanged.

`publish.unpublish` payload:

- `node_id`: `blog_post` node.
- `reason`: short reason code for history.

Acceptance removes public access and preserves authorized history.

## Shared Envelope Fields

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

## Reducer Rules

- Reducers are pure and deterministic.
- Reducers do not read redb, HTTP cookies, clocks, or global state.
- Applying the same accepted operation twice does not change projection twice.
- Restore operations create new history instead of removing prior operations.
