# File IO

Owner doc for visible file behavior.

## Open

- Open file creates or focuses a buffer for that path.
- Invalid UTF-8 opens as read-only diagnostic text until a richer view exists.
- Large files open in reduced-feature mode when needed.

## Save

- Save writes the active file-backed buffer.
- Save as binds a scratch or recovered buffer to a path.
- The app preserves detected line endings unless the user changes policy.

## External Changes

- If a clean buffer changes on disk, reload after confirmation.
- If a dirty buffer changes on disk, show a conflict notice.
- Deleted files keep their buffer open until the user saves elsewhere or closes it.
