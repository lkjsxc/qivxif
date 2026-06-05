# Resource Orchestration

## Purpose

Resource orchestration is qivxif's signature product behavior. It keeps direct
interaction instant while planning storage, cache, sync, media, thumbnails,
indexes, and background work as one coherent system.

## Contents

- [diagnostics.md](diagnostics.md): user-visible planner state.
- [settings.md](settings.md): user controls for budgets and pins.

## User Promise

- Dirty local data is protected.
- Active tab resources are protected.
- Pinned media is protected.
- Expensive work is scheduled outside pointer paths.
- Storage pressure produces explainable plans.
- Failed jobs remain inspectable.

## Managed Work

- Local storage usage and OPFS quota.
- Cache warming and eviction.
- Media chunk retention.
- Thumbnail generation.
- Graph index refresh.
- Graph map cache invalidation.
- Sync retry order.
- Service worker asset checks.
- Dirty-event protection.

## Rule

The planner computes the whole plan before any mutation runs.
