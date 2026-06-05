# Resource Diagnostics

## Surface

Diagnostics show:

- current planner status.
- last plan summary.
- protected bytes.
- prunable bytes.
- cache usage by resource class.
- queued jobs.
- failed jobs and reasons.
- active leases.
- recent journal entries.

## Explanations

Every protected or prunable resource can answer:

- why it exists.
- which resource class owns it.
- whether it is dirty, active, pinned, accepted, or expendable.
- which lease protects it.
- which plan may mutate it next.

## User Actions

- Run planner now.
- Pin or unpin a resource.
- Retry a failed job.
- Clear expendable previews.
- Inspect storage mode and quota.

## Safety

Diagnostics are real data from repositories. They do not invent resource rows
when storage is empty.
