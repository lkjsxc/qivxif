# Cache Orchestration

## Synthesis

The cache is a planned subsystem, not ad hoc browser storage.

## Requirements

- Dirty local operations dominate eviction.
- Pinned nodes survive normal cleanup.
- Workspace-hot records are prioritized.
- Graph-neighbor warming is bounded.
- Every eviction decision is journaled.
