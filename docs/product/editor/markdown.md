# Markdown

## Behavior

- Markdown is stored as plain text.
- Preview is a projection, not a separate durable rich text object.
- Blog publishing reads Markdown through public ACL projection.
- Rendering does not run inside redb write transactions.

## First Public Renderer

The first public renderer supports:

- escaped HTML text.
- paragraph blocks separated by blank lines.
- `#` and `##` heading blocks.

Unsupported Markdown syntax remains escaped paragraph text until documented.
