# Line Limits

Owner doc for file length constraints.

## Limits

- Markdown under `docs/`: 300 lines or fewer.
- Authored Rust, shell, YAML, and TOML checked by quality: 200 lines or fewer.
- Root support config must remain small.

## Practice

- Split before a file becomes hard to scan.
- Prefer ownership boundaries over arbitrary chunks.
- Do not compress names or remove clarity to pass the gate.
