# Backup Restore

## Rules

- Stop writes before copying the redb file.
- Include blob chunks with the database.
- Restore runs repair check before server start is considered healthy.
- Client IndexedDB data is separate from server backup.
