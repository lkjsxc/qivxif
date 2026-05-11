# ECS Boundary

## Direction

Use `bevy_ecs` as the standalone ECS inside region actors for region-local
state that benefits from dense queries and scheduled systems.

## Rules

- `bevy_ecs` is an internal region data model.
- `bevy_ecs` does not replace actor ownership.
- Cross-region state transfer is serialized through handoff messages.
- Tokio actors own region mailboxes and service orchestration.
- ECS systems may mutate only the region world they are scheduled inside.

## Initial Slice

`bevy_ecs` is already active for region-local stats. Gameplay mutation still
uses simple structs until entity behavior requires ECS systems.
