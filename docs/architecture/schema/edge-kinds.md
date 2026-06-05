# Edge Kinds

Edges are first-class durable records. Unknown kinds are rejected by API
validation, sync acceptance, and graph reducers.

## Registry

| Kind | From | To | Purpose |
| --- | --- | --- |
| `links_to` | any node | any node | generic user-created link |
| `contains` | container node | child node | composition or containment |
| `parent_of` | parent node | child node | tree parent relation |
| `ordered_child` | parent node | child node | tree relation with explicit order metadata |
| `references` | any node | any node | semantic reference |
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
| `placed_on_graph_map` | `graph_map_item` | `graph_map` | map membership |
| `media_attachment` | any node | `media_asset` | attached media relation |
| `tile_contains_pane` | `tile_layout` | `pane` | layout membership |
| `pane_views_node` | `pane` | any node | pane target |
| `supersedes` | replacement node | replaced node | projection replacement relation |
| `tombstones` | tombstone marker | target node | explicit tombstone relation |

## Required Edge Fields

- `id`.
- `from_node`.
- `to_node`.
- `kind`.
- `created_by`.
- `created_at`.
- `metadata_map`.
- `tombstone`.

## Rules

- Edges are never nested arrays inside nodes.
- Forward and reverse indexes update in the same write transaction.
- Tombstoned edges remain available to history and repair tools.
- Owner docs define metadata keys for typed edge behavior.
- Tree projections use explicit relation edges and never hidden child arrays.
- Edge event owner docs define event-to-event and edge-to-edge relation indexes.

## Tile Layout Metadata

`tile_contains_pane` metadata:

- `slot`: command-created slot label.

`pane_views_node` metadata:

- `pane_kind`: documented pane kind.

## Graph Map Metadata

`placed_on_graph_map` metadata:

- `placement_seq`: mirrors the graph-map item placement sequence.
- `position_key`: deterministic map ordering key when present.

`contains` metadata for graph-map items:

- `relation`: `graph_map_item_target`.

## Media Metadata

`media_attachment` metadata:

- `attachment_kind`: inline, avatar, cover, reference, or download.
- `caption`: optional user text.

## Publishing Metadata

`references_text` metadata:

- `relation`: `blog_body`.
