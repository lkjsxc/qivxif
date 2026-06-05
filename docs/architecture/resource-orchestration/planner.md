# Resource Planner

## Rule

Plan before mutate. A planner run produces a complete `ResourcePlan` from a
snapshot. Executors apply that plan through repositories and record journal rows.

## Resource Classes

- `dirty_event`.
- `accepted_event`.
- `text_snapshot`.
- `tab_snapshot`.
- `tile_layout`.
- `graph_index`.
- `graph_view_cache`.
- `media_original`.
- `media_chunk`.
- `media_thumbnail`.
- `service_worker_asset`.
- `feed_window`.
- `profile_cache`.
- `sync_cursor`.
- `upload_session`.
- `download_session`.
- `preview_cache`.

## Inputs

- usage and quota.
- OPFS availability.
- SQLite worker mode.
- dirty event count.
- sync status.
- active panes.
- pinned resources.
- media transfer state.
- graph map cache demand.
- network state.
- save-data and battery signals when available.
- recent errors.

## Outputs

- warm cache entries.
- evict cache entries.
- compact snapshots.
- thumbnail jobs.
- media chunk retention.
- sync retry order.
- graph index refresh.
- stale view cache invalidation.
- diagnostics.
- journal entries.

## Protection Rules

- Dirty local data is never evicted.
- Active tab resources have hard protection.
- User-pinned media has hard protection.
- Recently viewed resources have soft protection.
- Upload sessions survive refresh.
- Large media cache obeys explicit budget.

## Durable Tables

Browser repositories own:

- `resource_entries`.
- `resource_leases`.
- `resource_journal`.
- `resource_jobs`.

Server storage may mirror the same concepts for media serving and maintenance.
