# Edit Groups

## Purpose

Edit groups make undo, redo, and history inspection user-meaningful.

## Rules

- A group may contain multiple character-level CRDT operations.
- Undo and redo create operations.
- Group boundaries are controlled by editor events, pauses, and explicit commands.
