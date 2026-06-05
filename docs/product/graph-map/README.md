# Graph Map

## Purpose

Graph Map is the 2D relationship surface for qivxif. It replaces the retired
canvas term with a clearer projection over graph nodes and edges.

## Contents

- [projection.md](projection.md): visual projection and dimension toggles.
- [workflows.md](workflows.md): user flows for exploring and editing relations.

## Surface

- Nodes render as circles or compact glyphs.
- Edges render as lines between visible nodes.
- Users can zoom, pan, select, filter, and inspect graph records.
- Dimension toggles change the projection and never delete data.
- Selecting a node opens an inspector with metadata, edges, profile card when
  relevant, and actions.
- Double-click or command opens a node in an adjacent pane.

## Durable Truth

- Durable truth remains node records, edge records, and graph events.
- View state stores query shape, selected dimensions, pinned positions, zoom,
  pan, selection, and layout hints.
- Positions are graph-map metadata, not fields on the displayed node.
- Creating or deleting a relationship writes real edge records or tombstones.
- Automatic relationship suggestions write suggestion records until accepted.

## Bounds

- Query depth and node count are bounded.
- ACL-hidden nodes do not enter the projection.
- Edges render only when both endpoints are visible.
- Layout computation is cancellable.
- No sample graph appears on the main path.
