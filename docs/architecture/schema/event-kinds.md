# Event Kinds

Every durable mutation uses one of these event kinds. Unknown kinds are rejected
before reducer application.

## Registry

| Kind | Auth | Targets | Payload | Reducer |
| --- | --- | --- | --- | --- |
| `user.bootstrap_owner` | public setup | user | owner name, password hash, profile node | auth user insert |
| `auth.session_created` | public auth | session | user, session, csrf hash | session insert |
| `node.create` | write | node | kind, visibility, metadata | graph node insert |
| `node.update_metadata` | write | node | field patches | per-field register |
| `node.tombstone` | write | node | reason code | set tombstone |
| `edge.create` | link | edge, nodes | endpoints, kind, metadata | graph edge insert |
| `edge.tombstone` | link | edge | reason code | set tombstone |
| `edge.relate` | link | edge, event | relation kind, metadata | relation index insert |
| `text.create_doc` | write | node, text doc | text doc id | text doc insert |
| `text.insert` | write | text doc | position id, text | text CRDT insert |
| `text.delete` | write | text doc | range ids | text CRDT delete |
| `text.restore` | write | text doc | snapshot ref | append restore event |
| `tile.layout_set` | write | tile layout | complete tile tree | tile layout reducer |
| `board.item_place` | write | board, item | placement metadata | board projection reducer |
| `sync.cursor_advance` | write | cursor | cursor id, position | cursor update |
| `publish.post` | publish | blog post | slug, summary, public time | publication reducer |
| `publish.unpublish` | publish | blog post | reason code | publication reducer |
| `social.short_post_create` | write | short post | body, visibility, reply target | graph plus feed reducer |
| `social.follow` | write | profiles | target profile | edge create |
| `social.unfollow` | write | follow edge | edge id | edge tombstone |
| `social.mute` | write | profiles | target profile | edge create |
| `social.unmute` | write | mute edge | edge id | edge tombstone |
| `social.block` | write | profiles | target profile | edge create |
| `social.unblock` | write | block edge | edge id | edge tombstone |

## Shared Envelope Fields

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

## Reducer Rules

- Reducers are pure and deterministic.
- Reducers do not read redb, HTTP cookies, clocks, or global state.
- Applying the same accepted event twice does not duplicate projection writes.
- Restore events create new history instead of removing prior events.
- Timestamps are metadata only.

## Tile Layout Payload

`tile.layout_set` payload:

- `layout_node_id`: `tile_layout` node being changed.
- `layout`: complete tile tree snapshot.
- `maximized_pane_id`: optional pane node ID inside the tile tree.

The `layout.root` tree uses N-way splits:

- Split nodes contain `axis`, `children`, and `sizes`.
- Stack nodes contain `active` and `tabs`.
- See [../../product/tile-shell/layout-tree.md](../../product/tile-shell/layout-tree.md).

## Board Placement Payload

`board.item_place` payload:

- `board_node_id`: board receiving the placement.
- `item_node_id`: displayed graph node.
- `placement_node_id`: board item node for this visible placement.
- `position_key`: deterministic ordering key when needed.
- `x` and `y`: board coordinates.

Movement creates another placement event and tombstones or supersedes the older
placement relation when the owner doc requires a single visible item.
