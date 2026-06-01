# Snapshots

## Purpose

Snapshots reduce replay cost.

## Rules

- Snapshots are rebuildable materialized views.
- Snapshots never replace operation history.
- Snapshot thresholds are documented before tuning.
- `snapshot_text` records content after a specific operation id.
- `restore_text` converts snapshot content into a new `text.restore` operation payload.
