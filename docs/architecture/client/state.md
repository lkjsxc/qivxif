# State

## Actors

- AppShellActor.
- TileLayoutActor.
- ResourceOrchestrator.
- SQLiteRepositoryActor.
- SyncActor.
- TransportActor.
- PresenceActor.
- EditorActor.
- GraphMapActor.
- MediaActor.
- ProfileActor.
- FeedActor.
- PublicationActor.
- NotificationActor.

## Rule

Components send typed commands. They do not mutate browser storage directly.

## Tile Layout Actor State

The tile layout actor owns:

- active layout node id.
- active pane node id.
- tile tree snapshot.
- active Graph Map node id.
- selected Graph Map item node id.
- per-pane text drafts keyed by pane node id.
- per-pane scroll snapshots keyed by pane node id.
- per-node accepted text snapshots keyed by node id.

The actor persists state through SQLite repository messages and queued durable
events. UI controls render from actor state and local projections.

Per-pane drafts are local visible-tab state. Saving a text event writes the
shared text snapshot for the target node and clears only the saved pane draft.

Per-pane scroll snapshots are local visible-tab state. Switching tabs, moving a
tab, or reloading the shell restores the pane body to the last saved vertical
scroll offset without changing other panes that view the same resource.
