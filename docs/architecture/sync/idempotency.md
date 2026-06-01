# Idempotency

## Rules

- Event IDs are immutable.
- Re-uploading an accepted event returns the prior acceptance.
- Applying an event twice does not change projected state twice.
- Server stores enough acceptance state to answer duplicate pushes.
