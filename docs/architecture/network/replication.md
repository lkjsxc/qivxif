# Replication

## Server Duties

- Choose entities and chunks relevant to each player.
- Send authoritative state.
- Include correction data when prediction diverges.

## Client Duties

- Predict local movement.
- Reconcile against server state.
- Keep stale visual state disposable.

## Rule

Replication is not ownership. Region actors still own mutation.
