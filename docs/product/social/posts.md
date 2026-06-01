# Posts

## Objects

- short post node
- reply edge
- mention edge
- repost edge
- bookmark edge
- reaction edge

## Behavior

- Posts are graph nodes.
- Short posts use `short_post` nodes.
- Plain short-post bodies are node metadata under `body`.
- Replies and mentions are edges.
- Visibility is enforced by server ACL.
- Offline posts queue as operations.
