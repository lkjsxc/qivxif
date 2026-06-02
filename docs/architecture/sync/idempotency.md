# Idempotency

## Rules

- Event IDs are immutable.
- Re-uploading an accepted event with the same payload hash and envelope shape
  returns the prior acceptance.
- Re-uploading the same event ID with different payload bytes, hash, kind,
  actor, sequence, parents, or targets rejects the event.
- Duplicate accepted events return before reducer application, so projections do
  not change twice.
- Server stores enough acceptance state to answer duplicate pushes.
- Route-specific write APIs and batch sync use the same event acceptance rule.
