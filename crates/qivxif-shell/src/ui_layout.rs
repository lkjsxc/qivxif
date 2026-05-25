use super::ShellApp;
use eframe::egui::{self, Color32, Stroke, Ui};
use qivxif_action::{AppCommand, PaneSpawn};
use qivxif_tiles::{PaneId, SplitAxis, TileNode};

pub(super) fn render_workspace(app: &mut ShellApp, ui: &mut Ui) {
    let root = match app.model.session.layout.maximized {
        Some(id) => TileNode::Leaf {
            panes: vec![id],
            active: 0,
        },
        None => app.model.session.layout.root.clone(),
    };
    render_node(app, ui, root);
}

fn render_node(app: &mut ShellApp, ui: &mut Ui, node: TileNode) {
    match node {
        TileNode::Leaf { panes, active } => render_leaf(app, ui, panes, active),
        TileNode::Split {
            axis,
            ratio,
            first,
            second,
        } => render_split(app, ui, axis, ratio, *first, *second),
    }
}

fn render_split(
    app: &mut ShellApp,
    ui: &mut Ui,
    axis: SplitAxis,
    ratio: f32,
    first: TileNode,
    second: TileNode,
) {
    let available = ui.available_size();
    match axis {
        SplitAxis::Vertical => {
            ui.horizontal(|ui| {
                ui.set_width((available.x * ratio).max(220.0));
                render_node(app, ui, first);
                ui.separator();
                render_node(app, ui, second);
            });
        }
        SplitAxis::Horizontal => {
            ui.vertical(|ui| {
                ui.set_height((available.y * ratio).max(160.0));
                render_node(app, ui, first);
                ui.separator();
                render_node(app, ui, second);
            });
        }
    }
}

fn render_leaf(app: &mut ShellApp, ui: &mut Ui, panes: Vec<PaneId>, active: usize) {
    let active_pane = panes.get(active).copied();
    let stroke = match active_pane == Some(app.model.session.focused_pane) {
        true => Stroke::new(1.5, Color32::from_rgb(119, 183, 255)),
        false => Stroke::new(1.0, Color32::from_rgb(48, 54, 70)),
    };
    egui::Frame::default()
        .fill(Color32::from_rgb(23, 26, 34))
        .stroke(stroke)
        .inner_margin(egui::Margin::same(6))
        .show(ui, |ui| {
            render_tabs(app, ui, &panes);
            let pane = panes
                .get(active)
                .and_then(|id| app.model.session.pane(*id))
                .cloned();
            match pane {
                Some(pane) => app.render_pane(ui, pane),
                None => {
                    ui.label("No pane");
                }
            };
        });
}

fn render_tabs(app: &mut ShellApp, ui: &mut Ui, panes: &[PaneId]) {
    ui.horizontal_wrapped(|ui| {
        for pane_id in panes {
            if let Some(pane) = app.model.session.pane(*pane_id).cloned() {
                let selected = app.model.session.focused_pane == *pane_id;
                if ui.selectable_label(selected, pane.title).clicked() {
                    app.model.session.focus(*pane_id);
                }
            }
        }
        if ui.button("x").clicked() {
            app.dispatch(AppCommand::CloseFocusedPane);
        }
        if ui.button("+").clicked() {
            app.dispatch(AppCommand::TabFocused(PaneSpawn::Editor {
                buffer_id: app.model.session.focused_editor_buffer().map(|id| id.raw()),
            }));
        }
    });
}
