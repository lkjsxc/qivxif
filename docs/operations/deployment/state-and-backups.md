# State And Backups

## State

The server data directory contains authoritative hot state.

## Backups

- Stop or flush before snapshot.
- Archive replay tail when available.
- Restore into a fresh directory for drills.

## Rule

Stateful verification uses disposable Compose volumes.
