# Blog

## Model

A blog post is a graph node with title, slug, summary, author, tags, body text node, publication state, and public timestamps.

## Behavior

- Draft, publish, unpublish, and slug changes are operations.
- Public rendering reads through ACL projection.
- Tags and author pages are graph projections.
- Dynamic public routes come before static render caching.
