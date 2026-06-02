mod insert;
mod shape;

use crate::{GraphError, GraphResult, TileLayout, TileTab};
use qivxif_core::NodeId;
use shape::{
    activate, append_to_stack, clear_maximized, ensure_contains, reject_existing, remove_tab,
    resize_split, split_stack,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SplitDirection {
    Left,
    Right,
    Top,
    Bottom,
}

pub fn focus_tab(mut layout: TileLayout, pane_id: &NodeId) -> GraphResult<TileLayout> {
    if !activate(&mut layout.root, pane_id) {
        return Err(GraphError::PaneMissing);
    }
    Ok(layout)
}

pub fn open_tab(
    mut layout: TileLayout,
    target_pane_id: &NodeId,
    tab: TileTab,
) -> GraphResult<TileLayout> {
    reject_existing(&layout.root, &tab.pane_node_id)?;
    if !append_to_stack(&mut layout.root, target_pane_id, tab) {
        return Err(GraphError::PaneMissing);
    }
    layout.maximized_pane_id = None;
    Ok(layout)
}

pub fn close_tab(layout: TileLayout, pane_id: &NodeId) -> GraphResult<TileLayout> {
    let (root, removed) = remove_tab(layout.root, pane_id);
    if removed.is_none() {
        return Err(GraphError::PaneMissing);
    }
    Ok(TileLayout {
        root: root.ok_or(GraphError::TileLayoutEmpty)?,
        maximized_pane_id: clear_maximized(layout.maximized_pane_id, pane_id),
    })
}

pub fn split_tab(
    mut layout: TileLayout,
    target_pane_id: &NodeId,
    tab: TileTab,
    direction: SplitDirection,
) -> GraphResult<TileLayout> {
    reject_existing(&layout.root, &tab.pane_node_id)?;
    let (root, changed) = split_stack(layout.root, target_pane_id, tab, direction);
    if !changed {
        return Err(GraphError::PaneMissing);
    }
    layout.root = root;
    layout.maximized_pane_id = None;
    Ok(layout)
}

pub fn move_tab_to_stack(
    layout: TileLayout,
    source_pane_id: &NodeId,
    target_pane_id: &NodeId,
) -> GraphResult<TileLayout> {
    if source_pane_id == target_pane_id {
        return focus_tab(layout, target_pane_id);
    }
    ensure_contains(&layout.root, target_pane_id)?;
    let (root, removed) = remove_tab(layout.root, source_pane_id);
    let tab = removed.ok_or(GraphError::PaneMissing)?;
    let mut next = TileLayout {
        root: root.ok_or(GraphError::TileLayoutEmpty)?,
        maximized_pane_id: None,
    };
    if !append_to_stack(&mut next.root, target_pane_id, tab) {
        return Err(GraphError::PaneMissing);
    }
    Ok(next)
}

pub fn move_tab_to_edge(
    layout: TileLayout,
    source_pane_id: &NodeId,
    target_pane_id: &NodeId,
    direction: SplitDirection,
) -> GraphResult<TileLayout> {
    if source_pane_id == target_pane_id {
        return focus_tab(layout, target_pane_id);
    }
    ensure_contains(&layout.root, target_pane_id)?;
    let (root, removed) = remove_tab(layout.root, source_pane_id);
    let tab = removed.ok_or(GraphError::PaneMissing)?;
    let (root, changed) = split_stack(
        root.ok_or(GraphError::TileLayoutEmpty)?,
        target_pane_id,
        tab,
        direction,
    );
    if !changed {
        return Err(GraphError::PaneMissing);
    }
    Ok(TileLayout {
        root,
        maximized_pane_id: None,
    })
}

pub fn maximize_pane(mut layout: TileLayout, pane_id: &NodeId) -> GraphResult<TileLayout> {
    ensure_contains(&layout.root, pane_id)?;
    layout.maximized_pane_id = Some(pane_id.clone());
    Ok(layout)
}

pub fn restore_maximized(mut layout: TileLayout) -> TileLayout {
    layout.maximized_pane_id = None;
    layout
}

pub fn resize_split_layout(
    mut layout: TileLayout,
    pane_id: &NodeId,
    sizes: Vec<u16>,
) -> GraphResult<TileLayout> {
    if !resize_split(&mut layout.root, pane_id, sizes) {
        return Err(GraphError::PaneMissing);
    }
    Ok(layout)
}

#[cfg(test)]
mod tests;
