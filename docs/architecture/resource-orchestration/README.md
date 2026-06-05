# Resource Orchestration Architecture

## Contents

- [planner.md](planner.md): plan-before-mutate contract.
- [jobs.md](jobs.md): queue and execution states.

## Ownership

The orchestrator owns cross-resource planning. Repositories own durable records.
Svelte surfaces show plan state and dispatch commands, but never mutate cache,
media, storage, or sync state directly.
