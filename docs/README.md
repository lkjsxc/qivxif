# Documentation Canon

`docs/` is the only active canon for qivxif product behavior, architecture,
operations, and repository rules.

## Global Rules

1. Update docs before implementation.
2. Keep one owner doc for each durable contract.
3. Keep every docs directory to one `README.md` plus multiple children.
4. Keep every Markdown file at 300 lines or fewer.
5. Keep every authored source file at 200 lines or fewer.
6. Prefer short declarative facts over narrative.
7. Optimize structure for LLM retrieval.
8. Remove retired contracts instead of keeping aliases.

## Top-Level Index

- [AGENTS.md](AGENTS.md): documentation-specific agent rules
- [active-work.md](active-work.md): current work lanes and next batches
- [vision/README.md](vision/README.md): purpose and principles
- [getting-started/README.md](getting-started/README.md): orientation and verification
- [product/README.md](product/README.md): player-visible behavior
- [architecture/README.md](architecture/README.md): runtime, network, simulation, world, persistence, and client contracts
- [operations/README.md](operations/README.md): deployment, verification, quality, and observability
- [repository/README.md](repository/README.md): layout, rules, and workflow
- [decisions/README.md](decisions/README.md): accepted, rejected, and open decisions
- [research/README.md](research/README.md): durable synthesis of research input

## Recursive Map

- `vision/`: purpose, principles, LLM authoring, product shape.
- `getting-started/`: orientation, quickstart, verification, where-next.
- `product/`
  - `player/`: camera, onboarding, progression, death and respawn.
  - `world/`: zones, terrain editing, starter basins.
  - `gameplay/`: bases, combat, inventory, events, economy.
  - `social/`: parties, chat, player markets.
- `architecture/`
  - `runtime/`: process model, services, task classes, observability.
  - `network/`: transport, sessions, lanes, replication, epochs, codecs, messages, security.
  - `simulation/`: region ownership, ECS boundary, ticks, handoff, authority.
  - `world/`: coordinates, terrain, chunk streaming, summaries, safe travel.
  - `persistence/`: hot state, archives, schemas, backup and restore.
  - `client/`: shells, renderer, streaming, prediction, mobile lifecycle.
- `operations/`
  - `deployment/`: Compose stack, runtime config, state and backups.
  - `verification/`: Compose pipeline, static gates, probes, goldens, soak, test stack.
  - `quality/`: line limits, documentation topology, acceptance gates.
  - `observability/`: tracing, profiling, incident response.
- `repository/`
  - `layout/`: root, docs, and workspace layout.
  - `rules/`: authoring, wording, line limits, dependency policy.
  - `workflow/`: docs-first sequence, commits, decision records.
- `decisions/`: accepted paths, rejected paths, open questions.
- `research/`: report synthesis plus networking, rendering, and storage references.

## Reading Order

1. [vision/purpose.md](vision/purpose.md)
2. [vision/product-shape.md](vision/product-shape.md)
3. [product/playable-target.md](product/playable-target.md)
4. [product/world/zones.md](product/world/zones.md)
5. [architecture/runtime/process-model.md](architecture/runtime/process-model.md)
6. [architecture/network/transport.md](architecture/network/transport.md)
7. [architecture/simulation/region-ownership.md](architecture/simulation/region-ownership.md)
8. [architecture/network/protocol-codecs.md](architecture/network/protocol-codecs.md)
9. [architecture/network/protocol-messages.md](architecture/network/protocol-messages.md)
10. [architecture/persistence/hot-state.md](architecture/persistence/hot-state.md)
11. [operations/verification/compose-pipeline.md](operations/verification/compose-pipeline.md)
12. [repository/workflow/docs-first-change-sequence.md](repository/workflow/docs-first-change-sequence.md)
