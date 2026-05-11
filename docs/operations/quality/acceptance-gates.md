# Acceptance Gates

## Rule

No behavior change is accepted without matching docs, implementation, and
verification.

## Gate Order

1. Owner docs.
2. Implementation.
3. Static gate.
4. Live probes when behavior is runtime-visible.
5. Evidence under `.sisyphus/evidence/`.
6. Commit.

## Reproducibility

- `.dockerignore` excludes repository history, local temp data, build output,
  and local redb files from verification builds.
- Cargo verification commands use `--locked`.
- The verify image pins `cargo-nextest`.
