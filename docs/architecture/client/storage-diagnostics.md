# Storage Diagnostics

## Purpose

Storage diagnostics make local persistence state visible without exposing worker
internals or raw database details.

## Diagnostic Record

```typescript
type StorageDiagnostics = {
  mode: "opfs" | "memory" | "unavailable";
  reason: string;
  quota: number;
  usage: number;
  pageCount?: number;
  inventory: StorageInventory;
  queue: EventQueueCounts;
  cache: CacheByteCounts;
  lastOperationError?: string;
};
```

## Mode Copy

| Mode | Copy rule |
| --- | --- |
| `opfs` | Normal local storage is active. |
| `memory` | Degraded local storage is active; reload may lose local changes. |
| `unavailable` | Local storage is unavailable and the reason must be visible. |

No product surface may display `indexeddb` as a local store mode.

## Inventory

Inventory reports row counts by repository family:

- workspace,
- events,
- dirty events,
- accepted events,
- nodes,
- edges,
- text snapshots,
- tab snapshots,
- tile layouts,
- feed windows,
- cache entries,
- cache journal,
- sync cursors.

## Queue Counts

Queue diagnostics report:

- dirty count,
- pending count,
- rejected count,
- accepted count.

Rejected entries remain visible in Sync Status until superseded by a real action.

## Cache Bytes

Cache diagnostics report:

- protected cache bytes,
- prunable cache bytes.

An empty cache ledger reports zero bytes, not absence filler text.

## Quota And Pages

- `quota` and `usage` come from the browser storage estimate when available.
- `pageCount` comes from SQLite when the worker can query it.
- Missing quota or page information is represented as zero or absent fields, not
  fake values.

## Error Handling

The worker records the latest storage operation error after failed requests.
Settings and Diagnostics render that value together with the mode and reason.
