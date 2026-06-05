# Profiles

## Purpose

Profiles are first-class graph nodes for user identity, authorship, follows,
mentions, avatar media, and public profile pages.

## Contents

- [editing.md](editing.md): profile view and edit behavior.
- [surfaces.md](surfaces.md): tabs, cards, search, and admin surfaces.

## Durable Model

- Every durable user owns exactly one `profile` node.
- `StoredUser` contains `profile_node_id`.
- Profile nodes are public graph records by default unless a user changes
  visibility through documented fields.
- Follow, mention, author, block, mute, and bookmark edges target profile nodes.
- Profile routes never expose password hashes, token secrets, or session secrets.
