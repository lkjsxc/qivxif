# Dialogs And Clipboard

Owner doc for platform dialogs and clipboard.

## Dialogs

- Open file and save file are commands that return paths.
- Dialog cancellation is not an error.
- Dialog results are routed back through typed effects.

## Clipboard

- Clipboard reads and writes are platform services.
- Text clipboard is required before rich clipboard.
- Clipboard failure creates a visible notice.
- Clipboard operations must not block rendering.
