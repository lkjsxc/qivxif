# Quickstart

Owner doc for first local commands.

## Inspect

```bash
git status --short --branch
cargo metadata --locked --no-deps --format-ver 1
```

## Check Docs

```bash
cargo run -p qivxifctl -- docs validate-topology
cargo run -p qivxifctl -- quality check-lines
cargo run -p qivxifctl -- quality check-wording
```

## Full Acceptance

```bash
./scripts/verify-compose.sh
```
