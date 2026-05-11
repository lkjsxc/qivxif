# Line Limits

LLM purpose: state file-size limits that keep agent context usable.

## Limits

| File type | Limit |
| --- | --- |
| Docs Markdown | `<=300` lines. |
| Authored Rust and shell source | `<=200` lines. |

## Rules

- Split by ownership.
- Do not minify.
- Do not remove useful names to save lines.
