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

The actor persists state through IndexedDB messages and queued durable
operations. UI controls render from actor state and local projections.
