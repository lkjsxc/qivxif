# Node Kinds

Node kinds are durable names. Unknown kinds are rejected by API validation, sync acceptance, and graph reducers.

## Registry

| Kind | Owner | Purpose |
| --- | --- | --- |
| `text` | text | editable plain-text document body |
| `blog_post` | publishing | draft and public article container |
| `short_post` | social | short social post body and state |
| `profile` | auth/social | user-facing identity node |
| `tag` | graph/social | reusable label node |
| `topic` | graph/social | subject grouping node |
| `graph_board` | boards | graph composition board |
| `board_item` | boards | board placement record when a node needs per-board state |
| `media` | publishing/social | uploaded media descriptor |
| `workspace_layout` | tile layout | durable tiled layout root |
| `pane` | tile layout | pane state in a layout |
| `feed_window` | social/offline | materialized feed range marker |

## Required Node Fields

- `id`
- `kind`
- `owner_user_id`
- `created_by`
- `created_at`
- `updated_at`
- `visibility`
- `acl_ref`
- `current_commit_group`
- `current_text_ref`
- `metadata_map`
- `tombstone`

## Rules

- Kind changes are new operations, not in-place rewrites.
- `metadata_map` may hold kind-specific fields documented by owner docs.
- Tombstoned nodes remain queryable through history and repair tools.
- Normal projections hide tombstoned nodes unless the query asks for history.

## Tile Layout Metadata

`workspace_layout` metadata:

- `title`: display label.
- `layout_json`: canonical JSON tile tree snapshot from `workspace.layout_set`.

`pane` metadata:

- `pane_kind`: documented pane kind.
- `title`: display label.

`board_item` metadata:

- `item_node_id`: displayed graph node.
- `x`: board x coordinate as decimal text.
- `y`: board y coordinate as decimal text.
- `placement_seq`: monotonic board placement sequence as decimal text.

## Publishing Metadata

`blog_post` metadata:

- `title`: display title.
- `body_node_id`: referenced text node.
- `slug`: public route slug.
- `summary`: public summary.
- `publication_state`: `draft`, `published`, or `unpublished`.
- `published_at`: server timestamp when published.
- `author_name`: server-owned route handle.
