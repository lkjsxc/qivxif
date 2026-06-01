# Moderation

## Behavior

- Mute hides content from the viewer.
- Block prevents interaction according to server rules.
- Public posts remain public unless ACL or moderation state changes.
- Replies inherit visibility constraints and may be more restrictive.
- Mute is one-way and hides target-author home-feed entries.
- Block is two-way for home-feed visibility and reply creation.

## Constraint

Moderation filters must apply before feed items leave the store/query boundary.
