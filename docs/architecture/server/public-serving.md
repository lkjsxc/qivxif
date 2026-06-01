# Public Serving

## Responsibilities

- Serve the browser app shell.
- Serve public author routes.
- Serve public blog post routes.
- Serve public tag routes.
- Reject private content at the query boundary.

## Rules

- Public rendering never bypasses ACL.
- Markdown render output can be cached after dynamic serving is correct.
