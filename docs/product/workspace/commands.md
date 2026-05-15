# Commands

Owner doc for user-facing commands.

## Required Commands

| Command | Default Shortcut |
|---|---|
| Command palette | Ctrl or Cmd P |
| Open file | Ctrl or Cmd O |
| Save | Ctrl or Cmd S |
| Save as | Ctrl or Cmd Shift S |
| New scratch buffer | Ctrl or Cmd N |
| Close pane tab | Ctrl or Cmd W |
| Split vertical | Ctrl or Cmd Backslash |
| Split horizontal | Ctrl or Cmd Shift Backslash |
| Toggle explorer | Ctrl or Cmd Shift E |
| Toggle Markdown preview | Ctrl or Cmd Shift M |
| New browser pane | Ctrl or Cmd Shift B |
| Find | Ctrl or Cmd F |
| Go to line | Ctrl or Cmd L |
| Undo | Ctrl or Cmd Z |
| Redo | Ctrl or Cmd Shift Z |

## Rules

- Commands route through a typed action layer.
- Panes may reject commands with a visible reason.
- Global shortcuts pause while a browser text field owns focus, except forced app commands.
