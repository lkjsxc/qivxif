# Wording

Owner doc for repository wording constraints.

## Preferred Terms

| Term | Use |
|---|---|
| Initial slice | Current limited implementation scope. |
| Protocol contract | Protocol compatibility marker. |
| Build contract | Build identity marker. |
| Schema contract | Storage or payload schema marker. |
| Deprecated path deleted | Retired paths are removed instead of aliased. |

## Avoid

| Wording | Reason |
|---|---|
| Named product-line labels | They become stale. |
| Preserved old-path aliases | They create parallel meanings. |
| Temporary parallel contracts | They obscure the owner doc. |
| Long speculative prose | It is hard to retrieve and verify. |

## LLM Notes

- Prefer short declarative facts.
- Use exact names from owner docs.
