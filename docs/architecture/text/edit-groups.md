# Edit Groups

## Purpose

Edit groups make undo, redo, and history inspection user-meaningful.

## Rules

- A group may contain multiple character-level CRDT events.
- Undo and redo create events.
- Group boundaries are controlled by editor events, pauses, and explicit commands.
