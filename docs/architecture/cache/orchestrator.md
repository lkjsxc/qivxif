# Orchestrator

## Loop

1. Measure quota and usage.
2. Load cache manifest.
3. Classify entries.
4. Protect dirty and pinned entries.
5. Choose warming targets.
6. Choose eviction candidates.
7. Write journal entry.
8. Execute mutations.
9. Write result entry.
10. Emit UI status.

## Rule

The plan is computed before storage is mutated.
