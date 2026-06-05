# Public Serving

## Responsibilities

- Serve the browser app shell.
- Serve public author routes.
- Serve public blog post routes.
- Serve public tag routes.
- Serve browser isolation headers required by worker-owned SQLite OPFS.
- Reject private content at the query boundary.

## Rules

- Public rendering never bypasses ACL.
- Static app responses set `Cross-Origin-Opener-Policy: same-origin` and
  `Cross-Origin-Embedder-Policy: require-corp` so the browser can use the OPFS
  SQLite path when available.
- Markdown render output can be cached after dynamic serving is correct.
