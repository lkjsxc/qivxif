# Backup Restore

## Rules

- Stop writes before copying the redb file.
- Include media blob directories with the database.
- Restore runs repair check before server start is considered healthy.
- Browser SQLite and OPFS data are client-local and separate from server backup.
