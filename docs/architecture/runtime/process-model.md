# Process Model

## Canon

One authoritative world server process owns protocol, sessions, simulation,
mutation, persistence handoff, and replication.

## Rules

- Clients send intent.
- Server owns truth.
- Region actors own mutable world state.
- Session code does not mutate world sections directly.
- Persistence code does not block region ticks.
