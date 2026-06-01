# Public Routes

## Routes

- `/@{handle}`
- `/@{handle}/{slug}`
- `/t/{tag}`
- `/t/{tag}/{slug}`

## Rules

- Private nodes never render through public routes.
- Unlisted content requires an exact public route.
- Unpublished content returns a structured not-found response.

## Blog Post Rendering

`GET /@{author}/{slug}`:

- finds one published `blog_post` for the author and slug.
- reads `body_node_id` from post metadata.
- renders body text as Markdown projection.
- does not open a redb write transaction.
- returns not found after unpublish.
