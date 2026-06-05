# Sync Status

## Visible State

- online or offline
- active sync lane
- queued local events
- last successful sync
- last rejection
- cache usage estimate
- dirty local count

## Rules

- Sync status is available as a pane.
- Conflicts are not hidden in subtle text.
- Server-required effects show `queued`, `pending validation`, `accepted`, or `rejected`.
- Publishing, ACL changes, login, and slug checks are never shown as accepted before server response.
- Every rejection links to the event id and error code.
- Non-accepted queue entries are listed in actor sequence order.
- Each queue row shows event kind, status, event id, route, and last error when
  present.
- Long ids and error details wrap inside the pane.
- Cache pressure warnings distinguish dirty protected data from evictable projections.

## API Mapping

- `accepted[]` from push clears dirty state for those events.
- `rejected[]` creates visible conflict or rejection rows.
- Pull progress updates last applied cursor only after reducers apply.
- Offline mode increments queued count only after local repository write succeeds.

## Route Flush Mapping

Route-specific flush responses update the same visible state:

- `POST /api/nodes` acceptance clears the matching `node.create` queue entry.
- `POST /api/edges` acceptance clears the matching `edge.create` queue entry.
- `POST /api/text/{node_id}/events` acceptance clears the matching text queue entry.
- `POST /api/publish/{node_id}` acceptance clears the matching publish entry and marks the post public.
- `POST /api/unpublish/{node_id}` acceptance clears the matching unpublish entry and removes public access.
- Any non-success envelope stores its error code on the queue entry and increments rejected count.
- Network failure leaves the event dirty and keeps queued count unchanged.
