# Workspace Layout

Owner doc for Rust workspace shape.

## Desired Members

- `apps/qivxif-superapp` for the desktop app.
- `apps/qivxifctl` for agent-facing checks.
- `crates/qivxif-shell` for native shell integration.
- `crates/qivxif-tiles` for layout logic.
- `crates/qivxif-editor-buffer` and `crates/qivxif-editor-view` for editor core.
- `crates/qivxif-workspace` and `crates/qivxif-persistence` for state.
- Pane crates for explorer, Markdown, and browser policy.

## Rule

Retired game-shaped members should be removed rather than kept as parallel canon.
