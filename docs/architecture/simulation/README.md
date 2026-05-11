# Simulation Architecture

Use this subtree for authoritative simulation facts.

## Current Implementation

- One `RegionHandle` actor is implemented.
- The actor receives commands over a Tokio `mpsc` channel.
- Chunk, mutation, and flush calls reply over `oneshot` channels.
- `bevy_ecs::World` stores region-local stats only.
- No fixed simulation tick loop exists yet.

## Child Index

- [region-ownership.md](region-ownership.md): region actor ownership.
- [ecs-boundary.md](ecs-boundary.md): ECS use inside the region actor.
- [tick-policy.md](tick-policy.md): implemented and dormant tick facts.
- [cross-region-handoff.md](cross-region-handoff.md): dormant handoff boundary.
- [authority-rules.md](authority-rules.md): server authority facts.
