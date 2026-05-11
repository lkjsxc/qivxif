# ECS Boundary

## Status

- Status: implemented for region-local stats only.
- Owner: `crates/qivxif-sim`.

## Implemented Facts

- `Region` contains a `bevy_ecs::World`.
- `RegionStats` is inserted as a resource.
- `place_block` increments `RegionStats::mutations`.
- `flush` increments `RegionStats::flushes`.
- Gameplay block mutation uses simple Rust structs, not ECS systems.

## Rules

- `bevy_ecs` is internal to a region actor.
- `bevy_ecs` does not replace actor ownership.
- Cross-region data must not share mutable ECS state.
- ECS systems may mutate only the region world they are scheduled inside.

## Not Implemented

- ECS component catalogs.
- ECS schedules for gameplay systems.
- Cross-region ECS handoff.
