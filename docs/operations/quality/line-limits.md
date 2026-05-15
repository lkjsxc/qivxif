# Line Limits

Owner doc for file length gates.

## Limits

| Area | Limit |
|---|---:|
| Markdown under `docs/` | 300 lines |
| Rust under `apps/` and `crates/` | 200 lines |
| Shell scripts under `scripts/` | 200 lines |
| Checked root config files | 200 lines |

## Rules

- Split files by owner boundary before shortening names.
- Do not minify prose or code to pass.
- Generated or external artifacts should stay out of checked paths.
