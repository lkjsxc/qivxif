# Edge Kinds

Edges are first-class durable records. Unknown kinds are rejected by API validation, sync acceptance, and graph reducers.

## Registry

| Kind | From | To | Purpose |
| --- | --- | --- | --- |
| `links_to` | any node | any node | generic user-created link |
| `contains` | container node | child node | composition or containment |
| `references_text` | content node | `text` | body text relation |
| `tagged_with` | any node | `tag` | tag projection |
| `authored_by` | content node | `profile` | author projection |
| `reply_to` | `short_post` | `short_post` | reply relation |
| `mentions` | content node | `profile` | mention relation |
| `reposts` | `short_post` | `short_post` | repost relation |
| `bookmarks` | `profile` | content node | bookmark relation |
| `reacts` | `profile` | content node | reaction relation |
| `follows` | `profile` | `profile` | social follow relation |
| `mutes` | `profile` | `profile` | one-way home-feed hiding relation |
| `blocks` | `profile` | `profile` | two-way interaction blocking relation |
| `placed_on_board` | any node | `graph_board` | board membership |
| `tile_contains_pane` | `tile_layout` | `pane` | layout membership |
| `pane_views_node` | `pane` | any node | pane target |

## Required Edge Fields

- `id`
- `from_node`
- `to_node`
- `kind`
- `created_by`
- `created_at`
- `metadata_map`
- `tombstone`

## Rules

- Edges are never nested arrays inside nodes.
- Forward and reverse indexes update in the same write transaction.
- Tombstoned edges remain available to history and repair tools.
- Owner docs define metadata keys for typed edge behavior.

## Tile Layout And Board Metadata

`tile_contains_pane` metadata:

- `slot`: command-created slot label.

`pane_views_node` metadata:

- `pane_kind`: documented pane kind.

`placed_on_board` metadata:

- `placement_seq`: mirrors the board item placement sequence.

`contains` metadata for board items:

- `relation`: `board_item_target`.

`references_text` metadata for publishing:

- `relation`: `blog_body`.
