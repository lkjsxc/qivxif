# Static Gates

Owner doc for static checks.

## Checks

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace --locked`
- `cargo run -p qivxifctl -- docs validate-topology`
- `cargo run -p qivxifctl -- quality check-lines`
- `cargo run -p qivxifctl -- quality check-wording`

## Rules

- Prefer locked dependency resolution.
- Report exact failing command.
- Do not rewrite files as part of a check command.
