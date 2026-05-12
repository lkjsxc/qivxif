# Static Gates

LLM purpose: enumerate non-live checks run by the verification image.

## Owner Scope

This file owns static gate names and command intent. The Compose wrapper order
is owned by [compose-pipeline.md](compose-pipeline.md).

## Stages

| Stage | Purpose |
| --- | --- |
| `cargo fmt -- --check` | Checks Rust formatting. |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` | Denies Clippy warnings. |
| `cargo nextest run --locked --workspace` | Runs workspace tests. |
| `cargo test --locked --doc --workspace` | Runs doctests. |
| `cargo build --locked --release --workspace` | Builds optimized artifacts. |
| `cargo run --locked --bin qivxifctl -- docs validate-topology` | Checks docs topology. |
| `cargo run --locked --bin qivxifctl -- quality check-lines` | Checks line limits. |
| `cargo run --locked --bin qivxifctl -- quality check-wording` | Checks banned wording. |

All Cargo inputs are locked.

## Output

Successful stages print compact `verify <stage> ... ok` lines.
`Dockerfile.verify` pins `cargo-nextest`; static commands use `--locked`.
Host-local Cargo commands are diagnostics only; Compose remains acceptance.
