# Active Work

## Current Lane

UI shell parity with lkjstr plus event-graph reset.

## Active Targets

- Keep docs canonical before behavior changes.
- Extend the real Axum and redb stack through product slices.
- Preserve offline browser event queues through each new user workflow.
- Add first-run owner setup inside the app shell.
- Make `/` render the shell immediately, even for an empty store.
- Reproduce the reference workspace grammar: split, stack, move, close, restore,
  and pane-local tab behavior.
- Keep each visible tab independent while sharing graph, text, feed, and cache
  resources by ID.
- Keep durable mutation language event-first across docs, Rust, API, and browser
  queues.
- Make edge events first-class enough to support tree and network projections.
- Keep board composition under qivxif-native wording.
- Add publishing, social feed, moderation, and transport slices on top of the graph model.
- Keep Docker Compose as the acceptance boundary.

## Stop Condition

The repo is coherent when all of these are true:

- `/` renders the shell immediately.
- Empty store opens Setup as a tab.
- Tabs can split, stack, move, close, and restore locally.
- Each visible tab has independent state.
- Every durable mutation is represented as an event with a random ID.
- Edge events exist and can form tree projections.
- Docker Compose verification passes.
- Another agent can read [README.md](README.md), run the Compose verification
  script, and continue from committed slices without hidden context.
