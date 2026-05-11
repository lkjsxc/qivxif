# Runtime Architecture

Use this subtree for server process and task boundaries.

## Current Implementation

- One authoritative server process exists.
- The server owns QUIC accept loops and sessions.
- Each connection gets one `Session` object.
- Region work runs in one actor task.
- Persistence calls execute through the region actor and storage crate.

## Child Index

- [process-model.md](process-model.md): authoritative process shape.
- [service-boundaries.md](service-boundaries.md): service ownership.
- [task-classes.md](task-classes.md): async, actor, and dormant tick work.
- [observability.md](observability.md): runtime signal contracts.
