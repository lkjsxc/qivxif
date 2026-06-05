# Profile Editing

## Editable Fields

- display name.
- handle or login name display.
- avatar media asset.
- bio.
- links.
- location or timezone when the user chooses.
- visibility.

## Read-Only Fields

- user id.
- actor id.
- roles visible to admins.
- created time.
- updated time.
- relationship counts.
- token metadata summary without secrets.

## Rules

- Edits write profile graph events.
- Avatar upload uses the media subsystem.
- Avatar attachment is a real edge from profile to media asset.
- Offline edits persist locally and sync later.
- Private fields require explicit owner or admin access.
