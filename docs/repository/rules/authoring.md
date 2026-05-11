# Authoring

Owner doc for documentation authoring rules.

## Rules

| Rule | Reason |
|---|---|
| Write docs before behavior code. | Missing docs are missing requirements. |
| Keep owner docs short. | Short files are easier for agents to retrieve. |
| Prefer exact names over broad prose. | Exact names reduce ambiguity. |
| Link owner docs instead of duplicating contracts. | One owner prevents parallel meanings. |
| Keep command examples executable. | Examples must remain verifiable. |

## Update Checklist

1. Find the owner doc for the contract.
2. Change the owner doc first.
3. Update parent `README.md` indexes when files move or split.
4. Keep every Markdown file under 300 lines.

## LLM Notes

- Preserve useful names; split files instead of minifying.
- Do not treat absent docs as permission to guess behavior.
