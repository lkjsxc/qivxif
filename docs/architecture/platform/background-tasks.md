# Background Tasks

Owner doc for async service work.

## Tasks

- Filesystem scans.
- Markdown parsing.
- Session writes.
- Recovery writes.
- Browser state updates.

## Rules

- Use typed channels between UI and workers.
- Long tasks support cancellation where feasible.
- Background failures are reported as structured notices.
- Shared mutable state requires narrow ownership.
