# Graph Map Projection

## Query Shape

A graph map query contains:

- center node id or saved graph-map node id.
- depth capped by product settings and server limits.
- node limit capped by product settings and server limits.
- enabled dimensions.
- edge kind filters.
- node kind filters.
- optional time range and author filters.

## Visual Rules

- Nodes are circles by default.
- Node color encodes selected kind or dimension only when the legend explains it.
- Edges are straight lines at first.
- Edge labels appear on selection or zoomed-in views.
- Hidden dimensions fade out or remove projected marks without mutating records.
- Selected node and selected edge have distinct focus treatment.

## Placement Records

A pinned placement records:

- graph map id.
- item node id.
- x and y coordinates.
- deterministic position key.
- pin owner.

Movement appends a placement event and supersedes older active placements for the
same graph map and item node.

## Dimension Toggles

Toggles cover:

- edge kind.
- node kind.
- author or profile.
- tag or topic.
- time range.
- media relation.
- publication relation.
- follow or profile relation.
- text reference and backlink.
- system and sync relation.

## Safety

- Projection never trusts client-only ACL.
- Local dirty edges may render with dirty state while awaiting sync.
- Tombstoned edges appear only in history or repair views.
- Layout workers may stop and return a bounded partial projection.
