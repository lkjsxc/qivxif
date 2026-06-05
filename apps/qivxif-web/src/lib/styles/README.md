# Styles

## Purpose

Own global CSS for the Svelte shell, panes, tabs, controls, and product surfaces.

## Allowed Imports

CSS files may import other CSS in this directory through app-level style entry
points only.

## Forbidden Imports

- TypeScript modules.
- Generated assets with hidden behavior.
- Layout state encoded as CSS-only product data.

## Owner Files

- `base.css`: reset, typography, and common tokens.
- `shell.css`: app header and frame.
- `panes.css`: split grid, pane body, resize, and drop overlays.
- `tabs.css`: tab rail, tab frames, drag classes.
- `surfaces.css`: tab surface layout.
- `graph-map.css`: Graph Map canvas and inspector layout.
- `controls.css`: shared form and button controls.

## Verification

Run web build and offline browser checks after interaction style changes.
