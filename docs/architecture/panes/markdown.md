# Markdown Architecture

Owner doc for Markdown preview internals.

## Pipeline

1. Read current buffer text snapshot.
2. Parse with `pulldown-cmark`.
3. Build an internal render model.
4. Map source ranges to preview blocks.
5. Render through egui-friendly widgets.

## Rules

- Preview does not own text.
- Parser work may run off the UI thread.
- Links are resolved through workspace policy.
- Snapshot tests cover render models.
