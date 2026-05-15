# Compose Pipeline

Owner doc for acceptance commands.

## Canonical Command

```bash
./scripts/verify-compose.sh
```

## Expected Scope

- Build verify image.
- Run formatting check.
- Run Clippy.
- Run unit and integration tests.
- Run docs topology, line-limit, and wording gates.
- Run desktop smoke checks when the shell exists.

## Rule

If Compose cannot run, report the blocker and run the narrow host checks that do not mutate implementation files.
