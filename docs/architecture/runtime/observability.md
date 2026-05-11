# Runtime Observability

## Required Signals

- `server starting`: includes config path, bind address, data directory,
  build epoch, and protocol epoch.
- `server shutdown`: emitted after the serve loop exits.
- `server listening`: includes the bound address.
- `connection accepted`: no raw client address at info level.
- `connection closed`: includes session identifier.
- `request handled`: includes session identifier and request name.
- `chunk request completed`: includes session identifier and returned cell count.
- `mutation accepted`: includes session identifier and request identifier.
- `mutation rejected`: includes session identifier and durable error code.
- `persistence flushed`: includes session identifier and request identifier.
- `persistence flush rejected`: includes session identifier, request identifier,
  and durable error code.
- `probe <name> ... ok`: emitted by `qivxifctl` after a successful probe.

## Dormant Signals

- `archive manifest written`: added when archive writes enter runtime paths.
- `archive manifest listed`: added when archive listing enters runtime paths.
- `region tick completed`: added when region ticks become externally observable.
- `request replayed`: added when replay diagnostics become part of acceptance.

## Direction

Use structured tracing. Logs must be compact enough for Compose gate failures.
