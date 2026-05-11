# Static Gates

## Stages

- Rust formatting.
- Clippy with warnings denied.
- Workspace tests through nextest.
- Doctests.
- Optimized build.
- Docs topology.
- Line limits.
- Locked Cargo inputs.

## Output

Successful stages print compact `verify <stage> ... ok` lines.
`Dockerfile.verify` pins `cargo-nextest`; static commands use `--locked`.
