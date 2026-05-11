# Commit Policy

Owner doc for repository commit cadence and batch shape.

## Rules

| Rule | Meaning |
|---|---|
| Commit frequently. | Keep review units small. |
| Keep commits coherent. | One commit should describe one related change set. |
| Do not mix unrelated behavior. | Split independent work into separate commits. |
| Do not commit `tmp/` research files. | `tmp/` is source material, not canon. |
| Include docs with behavior changes. | Docs and implementation must move together. |

## Suggested Batch Order

1. Docs canon.
2. Workspace and static verification.
3. Server smoke path.
4. Persistence probes.

## LLM Notes

- This file describes commit policy only; it is not permission to commit without request.
- Keep repository-doc-only changes separate from implementation changes.
