# Repository Workflow

Owner subtree for repository change order. Use it to keep docs, verification,
commits, and decisions aligned.

## Read Order

1. [docs-first-change-sequence.md](docs-first-change-sequence.md) for required order.
2. [commit-policy.md](commit-policy.md) for coherent batching.
3. [decision-records.md](decision-records.md) for durable decisions.

## Child Index

| Path | Purpose |
|---|---|
| [docs-first-change-sequence.md](docs-first-change-sequence.md) | Required docs-first change sequence. |
| [commit-policy.md](commit-policy.md) | Commit cadence and batch boundaries. |
| [decision-records.md](decision-records.md) | Decision record location and rules. |

## LLM Notes

- Change owner docs before implementation code.
- Do not commit `tmp/` research input as canon.
