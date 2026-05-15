# Purpose

qivxif is a Rust-native tile super app for local work.

## Goals

- Provide a fast native workspace for text editing, Markdown reading, file exploration, and controlled web browsing.
- Keep every durable contract in docs before implementation.
- Keep source modules small enough for LLM agents to read and edit safely.
- Prefer local files and explicit state over opaque services.

## Non-Goals

- No game, server, terrain, simulation, or network protocol canon remains.
- No browser-first shell.
- No plugin marketplace surface.
- No compatibility promise for retired paths.

## Source Of Truth

Docs are the durable truth. Reports and tmp files are input only until copied into owner docs.
