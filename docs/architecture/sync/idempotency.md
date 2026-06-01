# Idempotency

## Rules

- Operation IDs are immutable.
- Re-uploading an accepted operation returns the prior acceptance.
- Applying an operation twice does not change projected state twice.
- Server stores enough acceptance state to answer duplicate pushes.
