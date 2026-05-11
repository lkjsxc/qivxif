# Backup And Restore

## Status

- Status: restart persistence probe only.
- No full backup or restore workflow exists.

## Implemented Persistence Check

1. Probe places a block.
2. Probe sends `FlushPersistence`.
3. Compose restarts the server.
4. Probe requests the same chunk.
5. Probe expects the placed block in the returned cells.

## Implemented Storage Support

- `WorldStore` can reopen an existing database.
- Existing `WorldMeta` persists across reopen.
- Committed chunk overlays persist across reopen.

## Not Implemented

- Snapshot creation.
- Replay tail archiving.
- Restore into a fresh data directory.
- Profile continuity checks.
- Automated backup drills.
