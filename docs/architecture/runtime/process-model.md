# Process Model

## Status

- Status: implemented for one server process.
- Owner: `apps/qivxif-serverd::app`.

## Implemented Facts

- `serve` loads `ServerConfig` before opening network transport.
- `serve` opens `WorldStore` with configured `data_dir` and `world_seed`.
- `serve` spawns one `RegionHandle`.
- `serve` binds a Quinn endpoint to `bind_addr`.
- Each accepted connection runs in a Tokio task.
- Each connection owns one `Session`.
- Each accepted bidirectional stream handles one request.

## Rules

- Clients and probes send intent.
- Server owns protocol acceptance.
- Region actor owns terrain mutation.
- Session code does not mutate world storage directly.

## Not Implemented

- Multi-process world sharding.
- Native client runtime.
- Dedicated persistence worker pool.
