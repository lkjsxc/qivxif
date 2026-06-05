# Node Kinds

Node kinds are durable names. Unknown kinds are rejected by API validation, sync
acceptance, and graph reducers.

## Registry

| Kind | Owner | Purpose |
| --- | --- | --- |
| `text` | text | editable plain-text document body |
| `blog_post` | publishing | draft and public article container |
| `short_post` | social | short social post body and state |
| `profile` | auth/social | user-facing identity node |
| `tag` | graph/social | reusable label node |
| `topic` | graph/social | subject grouping node |
| `graph_map` | graph map | saved 2D relationship view |
| `graph_map_item` | graph map | view-local placement record |
| `media_asset` | media | uploaded media descriptor |
| `tile_layout` | tile layout | durable tiled layout root |
| `pane` | tile layout | pane state in a layout |
| `feed_window` | social/offline | materialized feed range marker |
| `resource_job` | resource orchestration | inspectable background job |

## Required Node Fields

- `id`.
- `kind`.
- `owner_user_id`.
- `created_by`.
- `created_at`.
- `updated_at`.
- `visibility`.
- `acl_ref`.
- `current_commit_group`.
- `current_text_ref`.
- `metadata_map`.
- `tombstone`.

## Rules

- Kind changes are new events, not in-place rewrites.
- `metadata_map` may hold kind-specific fields documented by owner docs.
- Tombstoned nodes remain queryable through history and repair tools.
- Normal projections hide tombstoned nodes unless the query asks for history.

## Tile Layout Metadata

`tile_layout` metadata:

- `title`: display label.
- `layout_json`: canonical JSON tile tree snapshot from `tile.layout_set`.

`pane` metadata:

- `pane_kind`: documented pane kind.
- `title`: display label.

## Graph Map Metadata

`graph_map` metadata:

- `title`: display label.
- `query_shape`: serialized bounded graph query.
- `dimension_state`: serialized enabled dimensions.

`graph_map_item` metadata:

- `item_node_id`: displayed graph node.
- `x`: map x coordinate as decimal text.
- `y`: map y coordinate as decimal text.
- `placement_seq`: monotonic placement sequence as decimal text.
- `position_key`: deterministic placement ordering key.

## Media Metadata

`media_asset` metadata:

- `content_hash`.
- `size`.
- `mime_type`.
- `filename`.
- `processing_state`.

## Publishing Metadata

`blog_post` metadata:

- `title`: display title.
- `body_node_id`: referenced text node.
- `slug`: public route slug.
- `summary`: public summary.
- `publication_state`: `draft`, `published`, or `unpublished`.
- `published_at`: server timestamp when published.
- `author_name`: server-owned route handle.
