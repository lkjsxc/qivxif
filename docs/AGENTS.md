# Documentation Agents

## Contract

- `docs/` is the durable source of truth for qivxif.
- Update owner docs before changing code.
- Keep one `README.md` directly under each docs directory.
- Each docs directory must contain multiple child docs or multiple child directories.
- Keep every docs file at 300 lines or fewer.
- Delete retired contracts instead of keeping aliases.
- Prefer short declarative facts and explicit ownership.

## Sequence

1. Read [README.md](README.md).
2. Find the smallest owner doc for the change.
3. Update that owner doc and its parent index.
4. Update decisions when a durable choice changes.
5. Run docs topology, line-limit, and wording gates.
