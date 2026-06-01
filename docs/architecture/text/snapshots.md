# Snapshots

## Purpose

Snapshots reduce replay cost.

## Rules

- Snapshots are rebuildable materialized views.
- Snapshots never replace event history.
- Snapshot thresholds are documented before tuning.
- `snapshot_text` records content after a specific event id.
- `restore_text` converts snapshot content into a new `text.restore` event payload.
