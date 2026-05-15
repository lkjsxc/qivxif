# Verification

Owner doc for agent verification.

## Preferred Path

Run the Compose acceptance script when implementation files are in scope:

```bash
./scripts/verify-compose.sh
```

## Docs-Only Path

For docs-only changes, run:

```bash
cargo run -p qivxifctl -- docs validate-topology
cargo run -p qivxifctl -- quality check-lines
cargo run -p qivxifctl -- quality check-wording
```

## Reporting

Always report commands run and any command that could not run.
