# Runtime Observability

## Status

- Status: implemented for tracing logs and probe stdout.
- Owner: `apps/qivxif-serverd`, `apps/qivxifctl`.

## Implemented Server Signals

| Signal | Implementation fact |
| --- | --- |
| `server starting` | Includes config path, bind address, data directory, build epoch, and protocol epoch |
| `server listening` | Includes bound address |
| `connection accepted` | Info log without raw client address |
| `connection remote address` | Debug log with raw client address |
| `request handled` | Includes session identifier and request name |
| `chunk request completed` | Includes session identifier and returned cell count |
| `mutation accepted` | Includes session identifier and request identifier |
| `mutation rejected` | Includes session identifier and durable error code |
| `persistence flushed` | Includes session identifier and request identifier |
| `persistence flush rejected` | Includes session identifier, request identifier, and durable error code |
| `connection closed` | Includes session identifier |
| `server shutdown` | Emitted after serve loop exits |

## Implemented Probe Signal

- `qivxifctl` prints `probe {label} ... ok` after a successful probe.

## Not Implemented

- Archive manifest runtime logs.
- Region tick completion logs.
- Request replay diagnostic logs.

## Rule

- Logs must stay compact enough for Compose gate failures.
