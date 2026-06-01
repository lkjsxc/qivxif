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
