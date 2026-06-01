# Blog

## Model

A blog post is a graph node with title, slug, summary, author, tags, body text node, publication state, and public timestamps.

## Durable Metadata

`blog_post` metadata:

- `title`: display title.
- `slug`: public route slug after publish.
- `summary`: public summary.
- `body_node_id`: text node used as Markdown body.
- `publication_state`: `draft`, `published`, or `unpublished`.
- `published_at`: server timestamp after publish.
- `author_name`: route handle supplied by the server.

## Behavior

- Draft, publish, unpublish, and slug changes are events.
- Public rendering reads through ACL projection.
- Tags and author pages are graph projections.
- Dynamic public routes come before static render caching.

## First Publish Flow

1. Create a `blog_post` node as a draft.
2. Link it to a text body through `references_text`.
3. Edit the text node as Markdown.
4. Queue `publish.post` with slug and summary.
5. Server validates slug uniqueness for the author.
6. Server marks the post public and records publish metadata.
7. Public route renders Markdown from the body text node.
