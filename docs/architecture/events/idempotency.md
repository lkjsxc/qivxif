# Idempotency

## Rules

- Event IDs are immutable.
- Re-uploading an accepted event with the same payload hash returns the prior
  acceptance.
- Re-uploading the same event ID with different payload bytes rejects the event.
- Applying an accepted event twice does not change projected state twice.
- Server stores enough acceptance state to answer duplicate pushes.
- Local collision retries happen only before a locally generated event is first
  accepted.
- Received event ID collisions are validation failures, not retry prompts.

## Cursor Rule

Sync cursors are server-owned resume tokens. They are not timestamps and do not
leak hidden event counts.
