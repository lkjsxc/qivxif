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
- [active-work.md](active-work.md): current work lanes.
- [architecture/README.md](architecture/README.md): system contracts.
- [decisions/README.md](decisions/README.md): durable choices.
- [getting-started/README.md](getting-started/README.md): agent orientation.
- [operations/README.md](operations/README.md): verification, quality, packaging, observability.
- [product/README.md](product/README.md): visible behavior.
- [repository/README.md](repository/README.md): layout, rules, workflow.
- [research/README.md](research/README.md): durable synthesis.
- [vision/README.md](vision/README.md): purpose and direction.

## Recursive Map

### architecture

- [architecture/README.md](architecture/README.md)
- [architecture/shell/README.md](architecture/shell/README.md), [architecture/shell/event-loop.md](architecture/shell/event-loop.md), [architecture/shell/rendering.md](architecture/shell/rendering.md), [architecture/shell/accessibility.md](architecture/shell/accessibility.md), [architecture/shell/command-routing.md](architecture/shell/command-routing.md)
- [architecture/editor/README.md](architecture/editor/README.md), [architecture/editor/buffer-core.md](architecture/editor/buffer-core.md), [architecture/editor/view-rendering.md](architecture/editor/view-rendering.md), [architecture/editor/recovery.md](architecture/editor/recovery.md)
- [architecture/panes/README.md](architecture/panes/README.md), [architecture/panes/pane-host.md](architecture/panes/pane-host.md), [architecture/panes/explorer.md](architecture/panes/explorer.md), [architecture/panes/markdown.md](architecture/panes/markdown.md), [architecture/panes/browser.md](architecture/panes/browser.md)
- [architecture/workspace/README.md](architecture/workspace/README.md), [architecture/workspace/layout-state.md](architecture/workspace/layout-state.md), [architecture/workspace/session-state.md](architecture/workspace/session-state.md), [architecture/workspace/settings-state.md](architecture/workspace/settings-state.md)
- [architecture/platform/README.md](architecture/platform/README.md), [architecture/platform/paths.md](architecture/platform/paths.md), [architecture/platform/dialogs-clipboard.md](architecture/platform/dialogs-clipboard.md), [architecture/platform/background-tasks.md](architecture/platform/background-tasks.md)

### product

- [product/README.md](product/README.md)
- [product/editor/README.md](product/editor/README.md), [product/editor/buffers.md](product/editor/buffers.md), [product/editor/cursor-selection.md](product/editor/cursor-selection.md), [product/editor/undo-redo.md](product/editor/undo-redo.md), [product/editor/file-io.md](product/editor/file-io.md)
- [product/workspace/README.md](product/workspace/README.md), [product/workspace/tile-layout.md](product/workspace/tile-layout.md), [product/workspace/commands.md](product/workspace/commands.md), [product/workspace/sessions.md](product/workspace/sessions.md), [product/workspace/focus-shortcuts.md](product/workspace/focus-shortcuts.md)
- [product/panes/README.md](product/panes/README.md), [product/panes/explorer.md](product/panes/explorer.md), [product/panes/markdown-preview.md](product/panes/markdown-preview.md), [product/panes/browser.md](product/panes/browser.md), [product/panes/settings.md](product/panes/settings.md)

### operations

- [operations/README.md](operations/README.md)
- [operations/quality/README.md](operations/quality/README.md), [operations/quality/acceptance-gates.md](operations/quality/acceptance-gates.md), [operations/quality/documentation-topology.md](operations/quality/documentation-topology.md), [operations/quality/line-limits.md](operations/quality/line-limits.md)
- [operations/verification/README.md](operations/verification/README.md), [operations/verification/compose-pipeline.md](operations/verification/compose-pipeline.md), [operations/verification/static-gates.md](operations/verification/static-gates.md), [operations/verification/smoke-tests.md](operations/verification/smoke-tests.md), [operations/verification/test-stack.md](operations/verification/test-stack.md)
- [operations/packaging/README.md](operations/packaging/README.md), [operations/packaging/desktop-artifacts.md](operations/packaging/desktop-artifacts.md), [operations/packaging/platform-deps.md](operations/packaging/platform-deps.md)
- [operations/observability/README.md](operations/observability/README.md), [operations/observability/tracing.md](operations/observability/tracing.md), [operations/observability/profiling.md](operations/observability/profiling.md), [operations/observability/incident-runbook.md](operations/observability/incident-runbook.md)

### repository

- [repository/README.md](repository/README.md)
- [repository/layout/README.md](repository/layout/README.md), [repository/layout/root-layout.md](repository/layout/root-layout.md), [repository/layout/docs-layout.md](repository/layout/docs-layout.md), [repository/layout/workspace-layout.md](repository/layout/workspace-layout.md)
- [repository/rules/README.md](repository/rules/README.md), [repository/rules/authoring.md](repository/rules/authoring.md), [repository/rules/wording.md](repository/rules/wording.md), [repository/rules/line-limits.md](repository/rules/line-limits.md), [repository/rules/dependency-policy.md](repository/rules/dependency-policy.md)
- [repository/workflow/README.md](repository/workflow/README.md), [repository/workflow/docs-first-change-sequence.md](repository/workflow/docs-first-change-sequence.md), [repository/workflow/commit-policy.md](repository/workflow/commit-policy.md), [repository/workflow/decision-records.md](repository/workflow/decision-records.md)

### decisions, research, vision, getting-started

- [decisions/README.md](decisions/README.md), [decisions/accepted.md](decisions/accepted.md), [decisions/rejected.md](decisions/rejected.md), [decisions/open-questions.md](decisions/open-questions.md)
- [research/README.md](research/README.md), [research/report-synthesis.md](research/report-synthesis.md), [research/desktop-stack.md](research/desktop-stack.md), [research/editor-stack.md](research/editor-stack.md), [research/browser-stack.md](research/browser-stack.md), [research/quality-stack.md](research/quality-stack.md)
- [vision/README.md](vision/README.md), [vision/purpose.md](vision/purpose.md), [vision/principles.md](vision/principles.md), [vision/product-shape.md](vision/product-shape.md), [vision/llm-authoring.md](vision/llm-authoring.md)
- [getting-started/README.md](getting-started/README.md), [getting-started/orientation.md](getting-started/orientation.md), [getting-started/quickstart.md](getting-started/quickstart.md), [getting-started/verification.md](getting-started/verification.md), [getting-started/where-next.md](getting-started/where-next.md)

## Reading Order

1. [vision/purpose.md](vision/purpose.md)
2. [vision/product-shape.md](vision/product-shape.md)
3. [product/workspace/tile-layout.md](product/workspace/tile-layout.md)
4. [product/editor/buffers.md](product/editor/buffers.md)
5. [architecture/shell/event-loop.md](architecture/shell/event-loop.md)
6. [architecture/editor/buffer-core.md](architecture/editor/buffer-core.md)
7. [architecture/workspace/session-state.md](architecture/workspace/session-state.md)
8. [architecture/panes/browser.md](architecture/panes/browser.md)
9. [operations/verification/compose-pipeline.md](operations/verification/compose-pipeline.md)
10. [repository/workflow/docs-first-change-sequence.md](repository/workflow/docs-first-change-sequence.md)
