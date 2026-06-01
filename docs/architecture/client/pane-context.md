# Pane Context

## Purpose

Pane-local commands must act on the pane that emitted them, not on ambient shell
state left behind by another tab.

## Context Fields

- `pane_id`: visible durable pane instance.
- `pane_kind`: renderer and command family.
- `target_node_id`: graph resource viewed by the pane, when present.

## Rules

- UI modules derive pane context from the active tab record.
- Header commands receive the active tab context for that stack.
- Body commands receive the tab context for the body that rendered them.
- Actors may use context to override `currentNodeId` or `activeBoardId` for one
  command.
- Shared resource caches remain keyed by graph node, text doc, edge, feed, and
  event IDs.
- Context overrides do not replace durable layout records.

## Current Mappings

- `text_editor` and `graph_node` set `currentNodeId`.
- `graph_board` sets `activeBoardId`.
- Non-resource panes leave resource IDs unchanged.
