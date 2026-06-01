# Repair

## Checks

- All tables from [redb-layout.md](redb-layout.md) exist.
- `meta.schema_contract` is known to the current binary.
- Every typed value decodes into its owner type.
- Node current pointers reference existing commit groups.
- Edge endpoints reference existing nodes or are tombstoned.
- Operation parents exist or are root references.
- Feed index entries reference readable nodes.
- Blob manifests reference existing chunks.
- Sync cursors point to known operation positions.
- Every secondary index points at an existing primary record.
- Every primary record that requires an index has the matching index entry.

## CLI

`qivxifctl store repair-check` prints a structured repair report.

## Error Behavior

- Decode failure is reported with table name, key bytes, and owner type.
- Missing primary records fail repair-check.
- Missing secondary indexes fail repair-check and may be rebuilt by an explicit repair command.
- Unexpected extra index entries fail repair-check.
- The default repair-check never mutates the store.
