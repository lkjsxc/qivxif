# State And Backups

LLM purpose: state what data is durable and how verification treats it.

## Owner Scope

This file owns operational handling of the server data directory. Persistence
format details belong in architecture persistence docs.

## State

The server data directory contains authoritative hot state.

## Backups

- Stop or flush before snapshot.
- Archive replay tail when available.
- Restore into a fresh directory for drills.

## Rule

Stateful verification uses disposable Compose volumes.
