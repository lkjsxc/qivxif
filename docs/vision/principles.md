# Principles

Owner doc for durable product and engineering rules.

## Product

- Start in the working surface, not a marketing page.
- Make panes predictable, rectangular, and recoverable.
- Keep text editing and local files central.
- Apply browser policy before any embedded, detached, or external route.

## Engineering

- Authored application logic and UI source are Rust-only.
- Use the custom qivxif tile engine for pane layout behavior.
- Keep pane implementations behind small boundaries.
- Use typed commands instead of ad hoc callbacks.
- Prefer simple data files for settings and workspace state.

## Maintenance

- Docs first, then implementation.
- One owner doc per durable contract.
- Delete retired contracts.
- Keep files small and directly named.
