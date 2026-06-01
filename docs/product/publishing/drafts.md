# Drafts

## Behavior

- Drafts can be created and edited offline.
- Offline publish intent waits for server validation.
- Slug conflicts produce a visible conflict surface.
- Unpublish keeps the node and history inspectable by authorized users.

## Draft Rules

- Draft nodes remain private until `publish.post` is accepted.
- Publish rejection does not remove draft edits.
- Slug conflicts keep the rejected `publish.post` queue entry visible in the
  Sync pane with event id and `publish.slug_conflict`.
- Unpublish sets `publication_state` to `unpublished` and removes public access.
- Restoring a prior public body creates new text events before another publish.
