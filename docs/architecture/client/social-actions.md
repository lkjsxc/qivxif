# Social Actions

## Local Queue Contract

Browser social controls create route-backed queue entries before network delivery.

Queued relationship actions:

- `social.follow`.
- `social.unfollow`.
- `social.mute`.
- `social.unmute`.
- `social.block`.
- `social.unblock`.

The entry stores:

- event id.
- actor sequence.
- request body.
- route path.
- dirty status.

## Profile Context

Follow, mute, and block commands use the current session user's `profile_node_id`
as the source profile. The source profile is displayed for inspection and is
never typed by the user.

The normal Social pane lists target profile candidates discovered from local
graph state. It does not expose a raw source profile field.

Unfollow, unmute, and unblock commands are shown beside existing relationship
edges. The user chooses a relationship row; the edge id is not typed into a form.

If no target profiles or relationship edges are available, the pane shows the
real disabled reason instead of invented sample actions.

## Local Projection

Create actions write a dirty edge projection into the graph repository so the
relationship is visible before sync. Clear actions mark the local edge dirty and
tombstoned until server acceptance.

Server acceptance replaces the dirty edge with the accepted edge payload.

## Rejection

Server rejection keeps the queue entry visible as rejected and leaves the local
edge dirty. The sync status pane owns the rejection message.
