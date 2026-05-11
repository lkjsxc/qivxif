# Docs Layout

Owner doc for `docs/` topology rules.

## Directory Rules

| Rule | Meaning |
|---|---|
| Every docs directory has one `README.md`. | Each subtree has a local navigation entrypoint. |
| Every docs directory has multiple children. | Avoid one-file subtrees that add navigation cost. |
| Parent `README.md` files index immediate children. | Do not skip levels in indexes. |
| Owner docs define behavior once. | Related docs may link but must not redefine contracts. |

## File Size

- Markdown files under `docs/` stay at 300 lines or fewer.
- Split by responsibility before removing useful names.
- Update parent indexes when a split creates, moves, or deletes files.

## LLM Notes

- Prefer compact tables for indexes.
- Treat `tmp/` material as source input, not durable canon.
