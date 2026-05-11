# Purpose

qivxif is a persistent voxel MMORPG survival sandbox built as a Rust-native
system.

## Source Of Truth

`docs/` is the durable canon. Reports under `tmp/` are research input only until
their findings are copied into owner docs.

## Goals

- Let players explore, build, fight, trade, and reshape a persistent world.
- Keep the server authoritative for movement, combat, inventory, claims, and world mutation.
- Keep the world understandable to LLM agents through compact docs and small source files.
- Prefer one coherent architecture over competing prototypes.
- Deliver native full-parity desktop and mobile clients after the server slice is stable.

## Non-Goals

- Do not build a browser-first game.
- Do not preserve old contracts after better ones land.
- Do not split into separate low-end and high-end engines.
- Do not treat client prediction, cached archives, or generated terrain as gameplay truth.
