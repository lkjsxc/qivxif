# Current State

Implementation snapshot for native workspace work.

| Area | Current contract | Implementation owner | Verification |
| --- | --- | --- | --- |
| Shell reducer | Commands reduce to explicit transitions with effects. | `crates/qivxif-shell` | reducer unit tests |
| Workspace layout | Native start opens explorer, editor, markdown, browser, and settings. | `crates/qivxif-workspace` | smoke and JSON round trip |
| Editor | Buffers track dirty state, revisions, undo history, save, and recovery. | `crates/qivxif-editor-buffer`, `crates/qivxif-shell` | editor and recovery tests |
| Explorer | Tree state is pure; filesystem scans are applied as results. | `crates/qivxif-explorer` | temp-dir scan tests |
| Markdown | Preview follows the focused editor unless pinned to a source buffer. | `crates/qivxif-markdown`, `crates/qivxif-shell` | parser and shell tests |
| Browser | Portable builds use validated external-browser fallback. | `crates/qivxif-browser` | URL/history tests |
| Settings | Settings live in workspace state and persist as readable TOML. | `crates/qivxif-workspace`, `crates/qivxif-persistence` | TOML store tests |
| Packaging | Windows package verifies bundle, zip, and DLL discovery. | `scripts/package-windows.sh` | package verifier |

## Stop Conditions

- Documentation, source, and scripts stay under repository line limits.
- `scripts/verify-compose.sh` is the primary acceptance gate.
- Windows packaging must run through `docker compose -f docker-compose.package.yml run --rm --build package-windows`.
