# Undo And Redo

Owner doc for edit history behavior.

## Transactions

- Contiguous typing coalesces into one transaction.
- Cursor movement ends typing coalescing.
- Paste is one transaction.
- Delete of a selected range is one transaction.
- Save does not clear undo history.

## User Behavior

- Undo reverses the latest transaction for the active buffer.
- Redo reapplies an undone transaction until a new edit occurs.
- History is bounded by memory policy, not by a fixed operation count.
- Recovery replay must not create user-visible undo noise.
