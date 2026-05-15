# Session State

Owner doc for workspace session state.

## Data

- Window state.
- Layout tree.
- Pane descriptors.
- Buffer descriptors.
- Focused pane.
- Recent files.

## Format

- Machine-written JSON.
- Includes a `schema_contract` string.
- Unknown pane kinds are retained only as diagnostic entries.
- Corrupt state never prevents app startup.
