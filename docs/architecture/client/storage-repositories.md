# Storage Repositories

## Purpose

Typed repositories are the only browser product boundary for durable local state.
They hide worker messages, SQLite connection details, OPFS, memory mode, and SQL.

## Rule

Product modules call repository methods. Svelte components do not import storage
modules. Repository calls return typed records or typed failures and never expose
raw SQL strings.

## Repository Families

| Repository | Durable ownership |
| --- | --- |
| Workspace layout | Active layout snapshot, layout node id, workspace markers |
| Event queue | Dirty, pending, rejected, and accepted local event records |
| Graph projection | Node and edge projections derived from local and accepted events |
| Text snapshot | Latest text snapshot per node for fast editor restore |
| Tab snapshot | Draft and scroll snapshot per durable pane id |
| Tile layout | Complete tile tree snapshot and active tile markers |
| Cache ledger | Protected and prunable cache records plus journal entries |
| Resource planner | Resource entries, leases, jobs, and planner journal |
| Media | Media metadata, chunk locators, upload sessions, and cache state |
| Profile | Editable profile snapshots and profile cache entries |
| Storage inventory | Counts and byte summaries by repository family |

## Method Groups

```typescript
type LocalRepositories = {
  workspace: WorkspaceRepository;
  events: EventQueueRepository;
  graph: GraphProjectionRepository;
  text: TextSnapshotRepository;
  tabs: TabSnapshotRepository;
  tile: TileLayoutRepository;
  cache: CacheLedgerRepository;
  resources: ResourceRepository;
  media: MediaRepository;
  profiles: ProfileRepository;
  inventory: InventoryRepository;
  diagnostics: StorageDiagnosticsRepository;
};
```

## Transaction Ownership

- Multi-table writes are worker transactions.
- Queue records and projection updates are committed together when one mutation
  changes both.
- Accepted queue transitions move durable evidence into accepted storage before
  removing dirty state.
- Rejected queue records remain inspectable until an explicit user or sync
  action supersedes them.

## Workspace Repository

- `load()` returns the latest workspace snapshot or `undefined`.
- `save(snapshot)` stores the complete local workspace snapshot.

## Event Queue Repository

- `append(entry)` stores a dirty event before UI reports it as queued.
- `listAll()` returns all event queue records.
- `listNonAccepted()` returns dirty, pending, and rejected records in actor
  sequence order.
- `markPending(eventId)`, `markAccepted(eventId, payload)`, and
  `markRejected(eventId, error)` perform durable status transitions.

## Projection Repositories

- Graph methods upsert and query node and edge projections.
- Text methods store and restore latest text snapshots by node id.
- Tab methods store draft and scroll state by pane id.
- Tile methods store complete layout snapshots and shell markers.

## Resource, Cache, Media, And Inventory

- Cache records classify protected and prunable bytes.
- Resource records explain protection, leases, plans, and job state.
- Media records store metadata and OPFS chunk locators, not large bytes in SQLite.
- Profile records cache editable profile projections.
- Cache journal entries describe cache state changes.
- Inventory reads count rows by table and summarize cache and media bytes.

## Implementation Status

The IndexedDB adapter is deleted. `src/lib/storage/` now owns the
SQLite-worker-backed repository boundary. The next cleanup is replacing remaining
generic `LocalStore` calls in effect modules with repository-family methods when
that improves reducer and port clarity.
