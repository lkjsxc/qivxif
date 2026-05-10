# Agent Instructions

## Work Order

1. Read `docs/README.md` before changing behavior.
2. Update the owner doc before implementation.
3. Keep parent `README.md` indexes current.
4. Verify with Docker Compose before claiming completion.
5. Commit small coherent batches.

## Hard Limits

- Markdown files under `docs/`: `<=300` lines.
- Authored source files under `apps/`, `crates/`, and `scripts/`: `<=200` lines.
- Do not minify or remove useful names to fit limits.

## Wording

- Use protocol epoch, build epoch, or schema epoch.
- Avoid release-family labels and stale legacy aliases.
- Delete retired contracts instead of preserving parallel meanings.

## Protected Canon

`docs/` is authoritative. `tmp/` is research input only.
