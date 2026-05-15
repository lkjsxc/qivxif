# Wording

Owner doc for wording constraints.

## Preferred Terms

| Term | Use |
|---|---|
| Initial slice | Current limited implementation scope. |
| Build contract | Build identity marker. |
| Schema contract | Storage or payload schema marker in prose. |
| `schema_contract` | Field name for machine state. |
| Deprecated path deleted | Retired paths are removed instead of aliased. |

## Avoid

| Wording | Reason |
|---|---|
| Named product-line labels | They become stale. |
| Letter-plus-number labels | They imply parallel old contracts. |
| Preserved old-path aliases | They create parallel meanings. |
| Temporary parallel contracts | They obscure the owner doc. |
| Long speculative prose | It is hard to retrieve and verify. |

## LLM Notes

- Prefer short declarative facts.
- Use exact names from owner docs.
- `qivxifctl quality check-wording` enforces banned wording in docs and root-facing Markdown.
