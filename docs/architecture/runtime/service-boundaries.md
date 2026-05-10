# Service Boundaries

## Services

- Gateway accepts QUIC connections.
- Session service owns login, join, heartbeat, and player routing.
- Region service owns world mutation.
- Persistence service owns durable writes and reads.
- Probe service exposes machine-checkable behavior.

## Rule

Services communicate through typed messages or crate APIs, not shared global
mutable state.
