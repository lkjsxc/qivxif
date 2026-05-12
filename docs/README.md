# Documentation Canon

`docs/` is the LLM navigation root for qivxif.

## Rules

- Update docs before implementation.
- Keep one owner doc for each durable contract.
- Keep each docs subtree on one `README.md` plus children.
- Keep Markdown files at 300 lines or fewer.
- Keep authored source files at 200 lines or fewer.
- Prefer short declarative facts.
- Optimize for retrieval, not prose.
- Remove retired contracts instead of aliases.

## Root Index

- [AGENTS.md](AGENTS.md): docs-specific agent rules.
- [active-work.md](active-work.md): current work lanes and next batches.
- [architecture/README.md](architecture/README.md): system contracts.
- [decisions/README.md](decisions/README.md): durable choices.
- [getting-started/README.md](getting-started/README.md): agent orientation.
- [operations/README.md](operations/README.md): deployment, verification, quality, observability.
- [product/README.md](product/README.md): player-visible behavior.
- [repository/README.md](repository/README.md): layout, rules, workflow.
- [research/README.md](research/README.md): durable synthesis of non-canon input.
- [vision/README.md](vision/README.md): purpose and direction.

## Recursive Map

### `architecture/`

- [README.md](architecture/README.md): system contracts.
- `current-slice/`
  - [README.md](architecture/current-slice/README.md): implemented authoritative server slice.
  - [vertical-loop.md](architecture/current-slice/vertical-loop.md): public path proven by Compose probes.
  - [ownership-map.md](architecture/current-slice/ownership-map.md): code owners for runtime contracts.
  - [contract-matrix.md](architecture/current-slice/contract-matrix.md): docs-to-code verification matrix.
  - [request-replay.md](architecture/current-slice/request-replay.md): duplicate mutating request behavior.
- `client/`
  - [README.md](architecture/client/README.md): native clients.
  - [headless-client.md](architecture/client/headless-client.md): implemented protocol client.
  - [gui-runtime.md](architecture/client/gui-runtime.md): desktop GUI runtime boundary.
  - [platform-shells.md](architecture/client/platform-shells.md): desktop and mobile shell rules.
  - [renderer.md](architecture/client/renderer.md): `wgpu` renderer family.
  - [asset-streaming.md](architecture/client/asset-streaming.md): streaming and cache rules.
  - [prediction-reconciliation.md](architecture/client/prediction-reconciliation.md): client prediction.
  - [mobile-lifecycle.md](architecture/client/mobile-lifecycle.md): mobile parity and lifecycle.
- `network/`
  - [README.md](architecture/network/README.md): transport, sessions, replication.
  - [transport.md](architecture/network/transport.md): QUIC transport contract.
  - [session-lifecycle.md](architecture/network/session-lifecycle.md): connection phases.
  - [message-lanes.md](architecture/network/message-lanes.md): reliable and latest-wins lanes.
  - [replication.md](architecture/network/replication.md): state delivery and correction.
  - [protocol-contract.md](architecture/network/protocol-contract.md): protocol and build contract gating.
  - [protocol-codecs.md](architecture/network/protocol-codecs.md): wire and archive codec ownership.
  - [protocol-messages.md](architecture/network/protocol-messages.md): public message catalog.
  - [security.md](architecture/network/security.md): transport, secrets, and admin proof.
- `persistence/`
  - [README.md](architecture/persistence/README.md): durable state.
  - [hot-state.md](architecture/persistence/hot-state.md): local authoritative database.
  - [object-archives.md](architecture/persistence/object-archives.md): snapshots and replay bundles.
  - [schema-contracts.md](architecture/persistence/schema-contracts.md): durable table ownership.
  - [backup-restore.md](architecture/persistence/backup-restore.md): recovery drills.
- `runtime/`
  - [README.md](architecture/runtime/README.md): server process and task boundaries.
  - [process-model.md](architecture/runtime/process-model.md): authoritative process shape.
  - [service-boundaries.md](architecture/runtime/service-boundaries.md): service ownership.
  - [task-classes.md](architecture/runtime/task-classes.md): async, tick, and bulk work.
  - [observability.md](architecture/runtime/observability.md): runtime signal contracts.
- `simulation/`
  - [README.md](architecture/simulation/README.md): authoritative simulation.
  - [region-ownership.md](architecture/simulation/region-ownership.md): region actor ownership.
  - [ecs-boundary.md](architecture/simulation/ecs-boundary.md): ECS use inside owned regions.
  - [tick-policy.md](architecture/simulation/tick-policy.md): fixed-step rules.
  - [cross-region-handoff.md](architecture/simulation/cross-region-handoff.md): entity transfer rules.
  - [authority-rules.md](architecture/simulation/authority-rules.md): server authority.
- `world/`
  - [README.md](architecture/world/README.md): world data and streaming.
  - [coordinate-model.md](architecture/world/coordinate-model.md): coordinates and sections.
  - [terrain-pipeline.md](architecture/world/terrain-pipeline.md): deterministic generation.
  - [chunk-streaming.md](architecture/world/chunk-streaming.md): interest and delivery.
  - [lod-and-summaries.md](architecture/world/lod-and-summaries.md): near, mid, and far rendering data.
  - [safe-spawn-and-teleport.md](architecture/world/safe-spawn-and-teleport.md): safe travel checks.

### `product/`

- [README.md](product/README.md): player-visible behavior.
- [playable-target.md](product/playable-target.md): first playable server target.
- `gameplay/`
  - [README.md](product/gameplay/README.md): survival and MMORPG systems.
  - [bases-claims.md](product/gameplay/bases-claims.md): base ownership and claims.
  - [combat-abilities.md](product/gameplay/combat-abilities.md): combat and abilities.
  - [inventory-crafting.md](product/gameplay/inventory-crafting.md): items and crafting.
  - [pvpve-events.md](product/gameplay/pvpve-events.md): contested events.
  - [economy.md](product/gameplay/economy.md): player-crafted economy.
- `player/`
  - [README.md](product/player/README.md): player controls, identity, progression, lifecycle.
  - [camera-controls.md](product/player/camera-controls.md): dual camera contract.
  - [gui-first-session.md](product/player/gui-first-session.md): first graphical protocol loop.
  - [onboarding.md](product/player/onboarding.md): first session flow.
  - [progression.md](product/player/progression.md): additive skill growth.
  - [death-respawn.md](product/player/death-respawn.md): death and recovery.
- `social/`
  - [README.md](product/social/README.md): social systems.
  - [parties-guilds.md](product/social/parties-guilds.md): group identity.
  - [chat.md](product/social/chat.md): communication surfaces.
  - [player-markets.md](product/social/player-markets.md): trade surfaces.
- `world/`
  - [README.md](product/world/README.md): player-facing world rules.
  - [zones.md](product/world/zones.md): sanctuary, starter, and frontier rules.
  - [terrain-editing.md](product/world/terrain-editing.md): permanent edit contract.
  - [starter-basins.md](product/world/starter-basins.md): onboarding spawn areas.

### `operations/`

- [README.md](operations/README.md): deployment, verification, quality, observability.
- `deployment/`
  - [README.md](operations/deployment/README.md): runtime deployment.
  - [compose-stack.md](operations/deployment/compose-stack.md): local Compose runtime.
  - [runtime-config.md](operations/deployment/runtime-config.md): config file contract.
  - [state-and-backups.md](operations/deployment/state-and-backups.md): state volumes and backups.
- `observability/`
  - [README.md](operations/observability/README.md): runtime signals and incident work.
  - [tracing.md](operations/observability/tracing.md): structured logs.
  - [profiling.md](operations/observability/profiling.md): CPU and GPU profiling direction.
  - [incident-runbook.md](operations/observability/incident-runbook.md): response outline.
- `quality/`
  - [README.md](operations/quality/README.md): structural quality gates.
  - [line-limits.md](operations/quality/line-limits.md): file limits.
  - [documentation-topology.md](operations/quality/documentation-topology.md): recursive docs structure.
  - [acceptance-gates.md](operations/quality/acceptance-gates.md): merge acceptance rules.
- `verification/`
  - [README.md](operations/verification/README.md): acceptance gates.
  - [compose-pipeline.md](operations/verification/compose-pipeline.md): canonical commands.
  - [static-gates.md](operations/verification/static-gates.md): static checks.
  - [protocol-probes.md](operations/verification/protocol-probes.md): live network probes.
  - [windows-demo-bundle.md](operations/verification/windows-demo-bundle.md): Windows server and client demo bundle.
  - [desktop-smoke.md](operations/verification/desktop-smoke.md): graphical client smoke gate.
  - [worldgen-goldens.md](operations/verification/worldgen-goldens.md): deterministic generation checks.
  - [render-goldens.md](operations/verification/render-goldens.md): renderer checks.
  - [soak-load.md](operations/verification/soak-load.md): long-running checks.
  - [test-stack.md](operations/verification/test-stack.md): nextest, doctest, snapshot, property, and benchmark tools.

### `repository/`

- [README.md](repository/README.md): layout, rules, workflow.
- `layout/`
  - [README.md](repository/layout/README.md): filesystem contracts.
  - [root-layout.md](repository/layout/root-layout.md): root paths.
  - [docs-layout.md](repository/layout/docs-layout.md): documentation tree.
  - [workspace-layout.md](repository/layout/workspace-layout.md): Rust workspace.
- `rules/`
  - [README.md](repository/rules/README.md): authoring constraints.
  - [authoring.md](repository/rules/authoring.md): writing rules.
  - [wording.md](repository/rules/wording.md): naming and wording rules.
  - [line-limits.md](repository/rules/line-limits.md): hard file limits.
  - [dependency-policy.md](repository/rules/dependency-policy.md): dependency choices.
- `workflow/`
  - [README.md](repository/workflow/README.md): change order.
  - [docs-first-change-sequence.md](repository/workflow/docs-first-change-sequence.md): required order.
  - [commit-policy.md](repository/workflow/commit-policy.md): commit cadence.
  - [decision-records.md](repository/workflow/decision-records.md): decision tracking.

### `decisions/`

- [README.md](decisions/README.md): durable choices.
- [accepted.md](decisions/accepted.md): chosen contracts.
- [rejected.md](decisions/rejected.md): inactive paths.
- [open-questions.md](decisions/open-questions.md): unresolved choices.

### `research/`

- [README.md](research/README.md): durable synthesis of non-canon input.
- [report-synthesis.md](research/report-synthesis.md): synthesis of research reports.
- [rendering-references.md](research/rendering-references.md): renderer findings.
- [networking-references.md](research/networking-references.md): network findings.
- [storage-references.md](research/storage-references.md): storage findings.
- [client-delivery-references.md](research/client-delivery-references.md): Windows and client delivery findings.
- [operations-references.md](research/operations-references.md): verification and tooling findings.
- [security-references.md](research/security-references.md): trust boundary findings.

### `vision/`

- [README.md](vision/README.md): purpose and direction.
- [purpose.md](vision/purpose.md): goals, non-goals, and source of truth.
- [principles.md](vision/principles.md): product, engineering, and maintenance guardrails.
- [product-shape.md](vision/product-shape.md): canonical game shape.
- [llm-authoring.md](vision/llm-authoring.md): LLM-oriented authoring constraints.

### `getting-started/`

- [README.md](getting-started/README.md): orient a new agent before editing.
- [orientation.md](getting-started/orientation.md): repository orientation.
- [quickstart.md](getting-started/quickstart.md): local Compose workflow.
- [verification.md](getting-started/verification.md): acceptance entrypoints.
- [where-next.md](getting-started/where-next.md): next reading paths.

## Reading Order

1. [vision/purpose.md](vision/purpose.md)
2. [vision/product-shape.md](vision/product-shape.md)
3. [product/playable-target.md](product/playable-target.md)
4. [architecture/runtime/process-model.md](architecture/runtime/process-model.md)
5. [architecture/network/transport.md](architecture/network/transport.md)
6. [architecture/simulation/region-ownership.md](architecture/simulation/region-ownership.md)
7. [architecture/network/protocol-codecs.md](architecture/network/protocol-codecs.md)
8. [architecture/network/protocol-messages.md](architecture/network/protocol-messages.md)
9. [architecture/persistence/hot-state.md](architecture/persistence/hot-state.md)
10. [operations/verification/compose-pipeline.md](operations/verification/compose-pipeline.md)
11. [repository/workflow/docs-first-change-sequence.md](repository/workflow/docs-first-change-sequence.md)
