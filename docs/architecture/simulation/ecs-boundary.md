# ECS Boundary

## Direction

Use `bevy_ecs` as the standalone ECS inside region actors when entity
complexity requires dense queries and scheduled systems.

## Rules

- `bevy_ecs` is an internal region data model.
- `bevy_ecs` does not replace actor ownership.
- Cross-region state transfer is serialized through handoff messages.
- Tokio actors own region mailboxes and service orchestration.
- ECS systems may mutate only the region world they are scheduled inside.

## Initial Slice

The initial chunk mutation path can use simple structs before the ECS boundary
is needed.
