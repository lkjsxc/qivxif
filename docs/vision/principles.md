# Principles

Owner doc for durable product and engineering rules.

## Product

- Start in the working surface, not a marketing page.
- Make panes predictable, rectangular, and recoverable.
- Keep text editing and local files central.
- Treat browser behavior as policy-bound and optional when a platform cannot embed it safely.

## Engineering

- Use Rust for application logic and UI shell.
- Keep pane implementations behind small boundaries.
- Use typed commands instead of ad hoc callbacks.
- Prefer simple data files for settings and workspace state.

## Maintenance

- Docs first, then implementation.
- One owner doc per durable contract.
- Delete retired contracts.
- Keep files small and directly named.
