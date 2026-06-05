# Graph Map Workflows

## Explore

- Open Graph Map from New Tab or a graph node action.
- Choose a center node or saved graph-map node.
- Expand one level of related nodes at a time.
- Toggle dimensions to isolate relationship families.
- Select a node to inspect metadata and nearby edges.

## Edit Relations

- Select two visible nodes.
- Choose an edge kind.
- Confirm the relationship.
- The app writes a local dirty `edge.create` record before showing it as queued.
- Tombstone removes a relationship from the active projection without erasing history.

## Open Adjacent Content

- Double-click a node to open it in an adjacent pane.
- Text nodes open in the editor.
- Profile nodes open in the profile surface.
- Media nodes open in the media inspector.
- Unknown node kinds show a durable metadata inspector.

## Pin Layout

- Drag a node to pin its position in the current graph map.
- Pins survive refresh and sync.
- Unpin returns the node to computed layout.
- Pinned positions do not change the displayed node record.

## Save View

A saved graph map stores query shape, dimension toggles, pins, zoom, pan, and
selected node. It does not duplicate visible graph data.
