# Conflict Surfaces

## Behavior

- Text conflicts are resolved through CRDT merge where possible.
- Publication conflicts are visible and require user action.
- ACL conflicts are server-authoritative.
- Field-map conflicts expose both candidates when deterministic choice is unsafe.
