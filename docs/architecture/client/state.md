# State

## Actors

- AppShellActor
- TileLayoutActor
- CacheOrchestrator
- IndexedDbStore
- SyncActor
- TransportActor
- PresenceActor
- EditorActor
- FeedActor
- PublicationActor
- NotificationActor

## Rule

Components send messages. They do not mutate IndexedDB directly.

## Tile Layout Actor State

The tile layout actor owns:

- active layout node ID.
- active pane node ID.
- tile tree snapshot.
- active board node ID.
- selected board item node ID.
- per-pane text drafts keyed by pane node ID.
- per-pane scroll snapshots keyed by pane node ID.
- per-node accepted text snapshots keyed by node ID.

The actor persists state through IndexedDB messages and queued durable
events. UI controls render from actor state and local projections.

Per-pane drafts are local visible-tab state. Saving a text event writes the
shared text snapshot for the target node and clears only the saved pane draft.

Per-pane scroll snapshots are local visible-tab state. Switching tabs, moving a
tab, or reloading the shell restores the pane body to the last saved vertical
scroll offset without changing other panes that view the same resource.
