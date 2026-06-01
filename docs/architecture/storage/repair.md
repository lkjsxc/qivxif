# Repair

## Checks

- Node current pointers reference existing commit groups.
- Edge endpoints reference existing nodes or are tombstoned.
- Operation parents exist or are root references.
- Feed index entries reference readable nodes.
- Blob manifests reference existing chunks.
- Sync cursors point to known operation positions.

## CLI

`qivxifctl store check` prints a structured repair report.
