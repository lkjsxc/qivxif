# Resource Settings

## Budgets

Settings expose user-tunable budgets for:

- media cache bytes.
- graph map cache entries.
- thumbnail bytes.
- feed window bytes.
- maximum background jobs.

## Pins

Users can pin:

- media originals they own.
- media cache copies they need offline.
- graph map views.
- text snapshots.
- profile cache entries.

## Scheduling

Users can choose whether background work favors:

- active workspace resources.
- media transfer completion.
- graph map responsiveness.
- sync catch-up.

## Rules

- Dirty events cannot be unprotected by settings.
- Active editor snapshots cannot be evicted by settings.
- Local owned media metadata remains durable even when cache chunks are pruned.
