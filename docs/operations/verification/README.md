# Verification

LLM purpose: find acceptance gates for static checks, live probes, goldens, and
long-running checks.

## Reading Order

1. [compose-pipeline.md](compose-pipeline.md) for full acceptance order.
2. [static-gates.md](static-gates.md) for build and docs gates.
3. [protocol-probes.md](protocol-probes.md) for live public-path probes.
4. Golden and soak files for specialized regression areas.

## Boundary

Verification docs define how behavior is checked. Owner docs define what the
behavior means.

## Child Index

- [compose-pipeline.md](compose-pipeline.md): canonical commands
- [static-gates.md](static-gates.md): static checks
- [protocol-probes.md](protocol-probes.md): live network probes
- [worldgen-goldens.md](worldgen-goldens.md): deterministic generation checks
- [render-goldens.md](render-goldens.md): renderer checks
- [soak-load.md](soak-load.md): long-running checks
- [test-stack.md](test-stack.md): nextest, doctest, snapshot, property, and benchmark tools
