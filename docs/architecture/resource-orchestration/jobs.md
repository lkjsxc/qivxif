# Resource Jobs

## Job States

- queued.
- leased.
- running.
- succeeded.
- failed.
- canceled.

## Job Kinds

- thumbnail generation.
- graph index refresh.
- graph map cache rebuild.
- media chunk prune.
- media chunk verify.
- upload retry.
- download retry.
- snapshot compact.
- service worker asset check.

## Leases

A lease records:

- resource id.
- class.
- holder.
- reason.
- priority.
- created time.
- expiry time when applicable.

Expired leases become soft until the planner renews or removes them. Hard
protection for dirty events, active resources, and pinned resources does not rely
on expiry alone.

## Journal

Every executor mutation records:

- plan id.
- resource id.
- action.
- prior class and protection.
- result.
- error code when failed.

Journal rows are bounded by retention settings and compacted only after they no
longer explain active failures.
