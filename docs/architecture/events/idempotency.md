# Idempotency

## Rules

- Event IDs are immutable.
- Re-uploading an accepted event with the same payload hash and envelope shape
  returns the prior acceptance.
- Re-uploading the same event ID with different payload bytes, hash, kind,
  actor, sequence, parents, or targets rejects the event.
- A duplicate accepted event returns before reducer application, so projections
  are not changed twice.
- Server stores enough acceptance state to answer duplicate pushes.
- Route-specific write APIs construct the same event envelope before using the
  shared idempotency rule.
- Local collision retries happen only before a locally generated event is first
  accepted.
- Received event ID collisions are validation failures, not retry prompts.

## Cursor Rule

Sync cursors are server-owned opaque resume tokens. They are not timestamps,
counters, or event IDs, and they do not leak hidden event counts.
