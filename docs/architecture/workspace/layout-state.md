# Layout State

Owner doc for tile tree state.

## Data

- Root tile node.
- Split axis and ratio for branch nodes.
- Tab stack and active tab for leaf nodes.
- Pane identity references.
- Custom qivxif tile engine metadata.

## Rules

- Closing a pane normalizes empty leaves.
- Split ratios are clamped to keep minimum pane sizes.
- Layout state must serialize to stable JSON.
- Layout tests cover split, close, move, and focus invariants.
- Third-party docking crate state is not part of the persisted contract.
