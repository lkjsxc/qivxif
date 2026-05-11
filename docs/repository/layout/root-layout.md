# Root Layout

Owner doc for top-level repository paths.

## Path Index

| Path | Role | LLM Handling |
|---|---|---|
| `README.md` | Project entrypoint. | Start here for human-facing overview. |
| `AGENTS.md` | Repository agent rules. | Follow before editing. |
| `docs/` | Authoritative canon. | Update owner docs before behavior code. |
| `apps/` | Binaries. | Implementation code; not changed by repository-doc edits. |
| `crates/` | Reusable Rust crates. | Implementation code; not changed by repository-doc edits. |
| `config/` | Runtime and verification config. | Use exact config names when referenced. |
| `scripts/` | Verification helpers. | Use exact command paths when referenced. |
| `tmp/` | Research input. | Non-canon; do not index as durable docs. |

## Rules

- Keep this file limited to top-level paths.
- Put documentation topology in [docs-layout.md](docs-layout.md).
- Put workspace membership in [workspace-layout.md](workspace-layout.md).
