# Browser Domain

Pure browser reducers live here. They transform local state snapshots and never
touch DOM, storage, HTTP, service workers, random IDs, or clocks.

## Contents

- [tile-tree.ts](tile-tree.ts): focus, open, close, split, maximize, and restore
  operations over the local tile tree.
- [tile-move.ts](tile-move.ts): move and reorder operations for dragged tabs.
- [tile-tab-update.ts](tile-tab-update.ts): New Tab insertion and conversion helpers.

## Rules

- Functions return new layout objects instead of mutating callers' state.
- Missing panes are reported with errors so UI actions can become no-ops.
- Visible tab identity is the durable pane node ID in the current shell.
- Domain behavior mirrors the Rust tile reducer contract.
