# Runtime Observability

## Required Signals

- Server startup config path.
- Bound address.
- Session connect and disconnect.
- Request handled.
- Chunk request count.
- Mutation accept and reject count.
- Persistence flush count.
- Probe result.

## Direction

Use structured tracing. Logs must be compact enough for Compose gate failures.
