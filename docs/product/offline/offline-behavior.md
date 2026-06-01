# Offline Behavior

## Works Offline

- Create nodes.
- Edit text.
- Draft posts.
- Change layouts.
- Browse cached neighborhoods.
- Inspect cached history.

## Rules

- Local operations are stored before UI marks them queued.
- Dirty local operations are never evicted.
- Server validation happens when connectivity returns.
