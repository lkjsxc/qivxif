# Tile Layout

Owner doc for visible tile behavior.

## Structure

- The tile super app layout is a tree of splits and tab stacks.
- The custom qivxif tile engine owns split, tab, drag, focus, and serialization rules.
- Each tab owns a stable pane identity.
- A leaf may contain one or more pane tabs.
- Split ratios persist across restart.

## Interactions

- Drag a tab to a side to split.
- Drag a tab to the center to stack.
- Splitters are resize handles.
- Closing the last tab removes that leaf and normalizes the tree.
- Maximize hides surrounding tiles until restored.

## Minimums

- General tile minimum: 220 px.
- Editor tile minimum: 320 px.
- Splitter thickness: 6 to 8 px.
- Dock threshold: 12 px.
