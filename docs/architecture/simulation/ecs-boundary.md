# ECS Boundary

## Direction

Use a standalone ECS inside region actors when entity complexity requires it.

## Rules

- ECS is an internal region data model.
- ECS does not replace actor ownership.
- Cross-region state transfer is serialized through handoff messages.

## Initial Slice

The initial chunk mutation path can use simple structs before ECS is needed.
