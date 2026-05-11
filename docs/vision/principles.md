# Principles

## Reader Model

Use this file as a compact checklist. Detailed contracts live in product,
architecture, operations, or decisions owner docs.

## Product

- Make the world persistent and consequential.
- Protect onboarding spaces, then allow dangerous frontier play.
- Make crafting, logistics, and player trade matter.
- Support dual camera play: social third-person and precision first-person.

## Engineering

- One authoritative server owns truth.
- Region actors own world mutation.
- Clients send intent and render prediction.
- Keep the initial architecture narrow: one server authority, one protocol
  library, one persistence boundary, and one renderer family.
- Persist accepted edits as authoritative hot state; treat generated terrain as
  disposable input.
- Verification runs through Docker Compose.

## Maintenance

- Docs are requirements.
- Small files are navigation aids, not style ornament.
- Prefer explicit contracts over inferred behavior.
- Translate research into owner docs before implementation.
