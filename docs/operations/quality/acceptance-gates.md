# Acceptance Gates

Owner doc for quality acceptance.

## Required Gates

- Docs topology passes.
- Line limits pass.
- Wording check passes.
- Rust formatting passes.
- Clippy passes with warnings denied.
- Tests pass through the Compose path when available.

## Rules

- Host-only checks are diagnostics.
- Compose is the acceptance boundary.
- A failing gate must be reported with exact command and failure scope.
