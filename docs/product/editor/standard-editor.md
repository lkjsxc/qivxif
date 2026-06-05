# Standard Editor

## Purpose

A text tab must be useful for ordinary writing while still mapping every durable
change into qivxif text events.

## Editing Baseline

- Text input works for single-line and multiline content.
- Selection, cursor movement, and keyboard navigation follow browser editor norms.
- Clipboard cut, copy, paste, and multiline paste work.
- Undo and redo operate on local edit groups.
- IME composition is respected; events are created after committed composition.
- Local edits persist before sync delivery.
- Refresh restores the latest local snapshot and dirty events.
- Two tabs viewing the same node keep independent cursor, scroll, and panel state.

## Keyboard Behavior

- Text shortcuts stay inside the editor while focus is in the editor.
- Global shell shortcuts do not intercept normal typing, selection, undo, redo,
  paste, search, or composition.
- Escape may leave editor-local search before returning focus to pane chrome.
- Save queues a durable text event and updates status only after local persistence.

## Durable Events

The editor adapter translates UI changes into text events:

- insert range.
- delete range.
- restore content.
- undo edit group.
- redo edit group.
- accepted remote merge.

Whole-document restore may be used while the character-id reducer stabilizes, but
it still writes `text.restore` and never replaces history silently.

## Repository Methods

The text repository owns:

- `loadTextSnapshot(nodeId)`.
- `saveTextSnapshot(nodeId, snapshot)`.
- `appendTextEvent(event)`.
- `listTextEvents(nodeId, cursor)`.
- `materializeText(nodeId)`.

## Status Line

The editor shows:

- saved.
- local dirty.
- queued.
- pending sync.
- accepted.
- rejected.
- offline.

It also shows word count, character count, current document title, and last local
persistence result.

## Search And Preview

- Search within the document highlights matches and supports next and previous.
- Markdown preview is a projection over plain text.
- Preview escapes raw HTML.
- The first renderer supports paragraphs plus `#` and `##` headings.
- Split preview state is pane-local and restorable.

## Accessibility

- The editable region has an accessible label derived from the node title.
- Search results and save state are announced without stealing focus.
- Contrast and focus indicators follow the shell visual contract.
