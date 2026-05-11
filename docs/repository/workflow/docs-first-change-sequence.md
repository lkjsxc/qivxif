# Docs-First Change Sequence

Owner doc for the required order of behavior-changing work.

## Sequence

| Step | Action |
|---:|---|
| 1 | Read existing owner docs. |
| 2 | Change owner docs to the desired behavior. |
| 3 | Update affected indexes. |
| 4 | Implement the behavior. |
| 5 | Add verification. |
| 6 | Run Docker Compose gates. |
| 7 | Commit the coherent batch. |

## Rule

Missing docs are missing requirements, not permission to guess silently.

## LLM Notes

- For documentation-only work, complete the docs and index steps, then verify line limits.
- For implementation work, do not skip the Docker Compose gates when they apply.
