# Cache Orchestrator

## Ownership

The cache orchestrator is now a lane inside the broader resource orchestrator.
The resource planner owns cross-resource decisions; cache code owns cache-specific
entry mutation and journal writes.

## Loop

1. Measure quota and usage.
2. Load cache manifest.
3. Classify entries.
4. Protect dirty, active, and pinned entries.
5. Choose warming targets.
6. Choose eviction candidates.
7. Return a plan to the resource orchestrator.
8. Execute accepted cache mutations through repositories.
9. Write result journal entries.
10. Emit UI status.

## Rule

The plan is computed before storage is mutated.

## Link

See [../resource-orchestration/planner.md](../resource-orchestration/planner.md)
for cross-resource planning.
