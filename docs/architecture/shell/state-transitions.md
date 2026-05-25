# Shell State Transitions

The shell owns command routing through a functional core.

## Types

- `reduce_shell(&ShellModel, CommandEnvelope) -> ShellTransition`.
- `ShellTransition` contains `state`, `effects`, and `events`.
- `ShellEffect` is the complete side-effect request list.

## Commands

- Pane commands: spawn, split, tab, close, focus, and maximize.
- Editor commands: new scratch, open path, edit buffer, undo, redo, save focused.
- Explorer commands: refresh root, toggle hidden, apply scan result, select path.
- Browser commands: navigate, back, forward, reload, open external.
- Settings commands: update safe fields and persist settings.

## Reducer Rules

- Reducers are deterministic and do not touch the filesystem, process launcher, or renderer.
- Reducers may create IDs and must reserve restored IDs before new IDs are created.
- Effects are explicit and executed by runtime helpers after reduction.
- Selectors expose focused editor buffer, markdown source, diagnostics summary, and paths.

## Effects

- `LoadFile`
- `SaveFile`
- `RefreshExplorer`
- `PersistWorkspace`
- `PersistSettings`
- `OpenExternalUrl`
- `WriteRecovery`
- `ClearRecovery`
- `RenderMarkdown`

## Tests

- Reducer tests assert state, events, and effects for each command family.
- Runtime tests cover effect execution separately.
