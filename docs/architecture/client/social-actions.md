# Social Actions

## Local Queue Contract

Browser social controls create route-backed queue entries before network delivery.

Queued relationship actions:

- `social.follow`
- `social.unfollow`
- `social.mute`
- `social.unmute`
- `social.block`
- `social.unblock`

The entry stores:

- operation id.
- actor sequence.
- request body.
- route path.
- dirty status.

## Profile Inputs

Follow, mute, and block forms accept a target profile node id. The current user's `profile_node_id` supplies the source profile and is never typed by the user.

Unfollow, unmute, and unblock forms accept the edge id to tombstone.

## Local Projection

Create actions write a dirty edge projection into IndexedDB so the relationship is visible before sync. Clear actions mark the local edge dirty and tombstoned until server acceptance.

Server acceptance replaces the dirty edge with the accepted edge payload.

## Rejection

Server rejection keeps the queue entry visible as rejected and leaves the local edge dirty. The sync status pane owns the rejection message.
