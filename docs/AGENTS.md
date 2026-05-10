# Documentation Agents

## Required Sequence

1. Find the owner doc for the behavior.
2. Update that owner doc first.
3. Update parent `README.md` links when files move.
4. Keep each Markdown file under 300 lines.
5. Run the docs topology and line-limit gates.

## Canon Rules

- A related doc may link to an owner doc but must not redefine its contract.
- `tmp/` reports are source material, not durable canon.
- Prefer compact bullets and tables.
- Keep names stable once implementation refers to them.

## Local Checks

Use Docker Compose for acceptance. Host commands are diagnostics only.
