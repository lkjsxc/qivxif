# View Rendering

Owner doc for editor view internals.

## Responsibilities

- Convert visible buffer ranges into visual lines.
- Shape text with font fallback and bidirectional support.
- Render caret, selection, gutters, and diagnostics.
- Keep scroll state per pane.

## Cache Keys

- Buffer identity.
- Buffer revision.
- View width.
- Font settings.
- Wrap and tab settings.

## Rules

- Layout cache may be dropped at any time.
- Input edits must update the buffer before repaint.
- IME composition is pane-local until committed.
