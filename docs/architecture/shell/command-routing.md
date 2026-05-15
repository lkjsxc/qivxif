# Command Routing

Owner doc for typed command routing.

## Layers

| Layer | Role |
|---|---|
| Shell | Converts menus and shortcuts into commands. |
| Dispatcher | Routes commands to workspace or focused pane. |
| Pane | Applies local behavior or rejects with a reason. |
| Background service | Handles async work and reports completion. |

## Rules

- Commands are typed Rust enums.
- Commands carry pane or buffer identity when needed.
- Command handlers return effects instead of mutating distant state directly.
- Rejected commands produce user-visible notices.
