# Product Shape

Owner doc for the canonical qivxif product.

## Primary Surface

The app opens to a native desktop workspace made of dockable tiles.

Each tile hosts one pane kind:

- editor
- explorer
- Markdown preview
- browser
- settings

## Core Loop

1. Open a folder or file.
2. Edit text in one or more panes.
3. Split or tab panes as work changes.
4. Preview Markdown beside its source.
5. Browse reference pages under explicit policy.
6. Restore the same workspace after restart.

## Quality Bar

- Startup must be deterministic enough for smoke checks.
- Text data must survive crashes through recovery state.
- Layout state must round trip through readable JSON.
- Settings must be readable TOML.
