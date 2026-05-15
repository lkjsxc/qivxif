# Recovery

Owner doc for data-loss prevention.

## State

- Recovery journals live outside source documents.
- Each dirty buffer writes append-only recovery records.
- Successful save truncates the matching recovery records.
- Scratch buffers use recovery records until saved or closed.

## Startup

- Detect recovery records before restoring normal session state.
- Show recovered buffers as explicit recovered entries.
- Never overwrite an on-disk document during automatic replay.
- Keep corrupt recovery records for diagnostics.
