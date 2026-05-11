# Worldgen Goldens

LLM purpose: state the deterministic terrain regression coverage.

## Purpose

Worldgen goldens keep deterministic terrain stable.

## Initial Check

The initial slice checks that the same seed and chunk coordinate return the same
block data plus chunk-scoped persisted overlays, including negative chunk
coordinates and air overrides.

## Rule

Generated terrain changes require owner docs and updated checks.
