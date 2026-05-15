# Profiling

Owner doc for performance investigation.

## Focus Areas

- Startup latency.
- Redraw cadence.
- Text shaping cost.
- Markdown parse cost.
- Filesystem scan cost.
- Browser embed overhead.

## Rules

- Profile before optimizing.
- Keep profiling hooks off by default.
- Record fixture size and platform when sharing results.
