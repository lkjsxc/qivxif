# Runtime Observability

## Required Signals

- `server starting`: includes config path, bind address, data directory,
  world seed, build epoch, and protocol epoch.
- `server listening`: includes the bound address.
- `connection accepted`: includes remote address.
- `connection closed`: includes session identifier.
- `request handled`: includes session identifier and request name.
- `chunk request completed`: includes session identifier and returned cell count.
- `mutation accepted`: includes session identifier and request identifier.
- `mutation rejected`: includes session identifier and durable error code.
- `persistence flushed`: includes session identifier and request identifier.
- `probe <name> ... ok`: emitted by `qivxifctl` after a successful probe.

## Direction

Use structured tracing. Logs must be compact enough for Compose gate failures.
