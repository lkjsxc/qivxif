# Static Gates

## Stages

- `cargo fmt -- --check` checks Rust formatting.
- `cargo clippy --locked --workspace --all-targets -- -D warnings` denies Clippy warnings.
- `cargo nextest run --locked --workspace` runs workspace tests.
- `cargo test --locked --doc --workspace` runs doctests.
- `cargo build --locked --release --workspace` builds optimized artifacts.
- `cargo run --locked --bin qivxifctl -- docs validate-topology` checks docs topology.
- `cargo run --locked --bin qivxifctl -- quality check-lines` checks line limits.
- Locked Cargo inputs.

## Output

Successful stages print compact `verify <stage> ... ok` lines.
`Dockerfile.verify` pins `cargo-nextest`; static commands use `--locked`.
Host-local Cargo commands are diagnostics only; Compose remains acceptance.
