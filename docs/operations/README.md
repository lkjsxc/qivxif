# Operations

LLM purpose: find runtime operation contracts for deployment, verification,
quality, and observability. This subtree does not define gameplay or protocol
behavior; it links to owner docs when behavior belongs elsewhere.

## Reading Order

1. Use the child index to select the operational concern.
2. Read the child `README.md` before leaf files.
3. Follow owner-doc links for protocol, world, or architecture details.

## Boundaries

- Deployment files describe local Compose, config, and durable state handling.
- Verification files describe acceptance gates and probe behavior.
- Quality files describe repository gates for docs and line limits.
- Observability files describe required logs, profiling direction, and incidents.

## Child Index

- [deployment/README.md](deployment/README.md): Compose and runtime configuration
- [verification/README.md](verification/README.md): acceptance gates
- [quality/README.md](quality/README.md): line limits and topology
- [observability/README.md](observability/README.md): traces, profiling, and incidents
