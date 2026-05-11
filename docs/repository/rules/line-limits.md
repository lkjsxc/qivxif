# Line Limits

Owner doc for hard repository file-size limits.

## Hard Limits

| Area | Limit |
|---|---:|
| Markdown under `docs/` | `<=300` lines |
| Authored source under `apps/`, `crates/`, and `scripts/` | `<=200` lines |

## Split Rules

- Split by responsibility.
- Keep tests focused.
- Do not minify.
- Do not remove useful names only to fit the limit.

## LLM Notes

- Check line counts after documentation edits.
- When splitting docs, update the nearest parent `README.md` child index.
