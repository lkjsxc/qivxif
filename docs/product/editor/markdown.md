# Markdown

## Behavior

- Markdown is stored as plain text.
- Preview is a projection, not a separate durable rich text object.
- Blog publishing reads Markdown through public ACL projection.
- Rendering does not run inside redb write transactions.
