# Operations References

## Source Reports

- `tmp/deep-research-report (47).md`
- `tmp/deep-research-report (49).md`
- `tmp/deep-research-report (50).md`
- `tmp/deep-research-report (51).md`

## Durable Findings

- Sibling repositories reinforce docs-first workflow, small files, recursive
  README navigation, and verified coherent batches.
- Docker Compose remains the acceptance boundary.
- Host-only checks are diagnostics, not acceptance.
- Evidence should be saved under `.sisyphus/evidence/` for agent tasks.
- Runtime observability should use `tracing` across request, storage,
  simulation, and verification paths.

## Testing Stack

| Tool | Intended boundary |
| --- | --- |
| nextest | Rust test execution |
| doctests | Public examples and API documentation |
| insta | Snapshot assertions |
| proptest | Property checks for state and protocol behavior |
| Criterion | Performance baselines |

## Owner Targets

| Finding | Durable owner |
| --- | --- |
| Compose acceptance | `operations/verification/compose-pipeline.md` |
| Line limits | `repository/rules/line-limits.md` |
| Evidence files | `getting-started/verification.md` |
| Runtime observability | `architecture/runtime/` |
