# Paths

Owner doc for path handling.

## Rules

- Use `PathBuf` for filesystem paths.
- Do not require UTF-8 for core path state.
- Store display labels separately from canonical paths.
- Compare canonical paths when available.
- Keep URLs separate from filesystem paths.

## App State

- Settings live in the app config directory.
- Workspace state lives in the app data directory.
- Recovery state lives under a bounded recovery directory.
- Downloads default to a user-visible downloads directory.
