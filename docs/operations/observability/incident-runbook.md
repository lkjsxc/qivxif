# Incident Runbook

Owner doc for local failure handling.

## Startup Failure

1. Capture command and platform.
2. Check settings and workspace state parsing.
3. Move corrupt state aside.
4. Re-run static gates.

## Data Loss Report

1. Preserve document, workspace state, and recovery records.
2. Avoid opening the app against the same state until copied.
3. Reproduce with a temporary directory.
4. Add a regression test before fixing.
