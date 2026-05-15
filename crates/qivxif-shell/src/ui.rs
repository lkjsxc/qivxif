use crate::{ShellModel, ShellSnapshot};
use eframe::egui::{self, Ui};
use qivxif_action::{AppCommand, CommandEnvelope, CommandSource, SplitDirection};
use qivxif_tiles::{PaneId, PaneKind, SplitAxis, TileNode};
use qivxif_workspace::PaneState;

#[path = "ui_panes.rs"]
mod ui_panes;

pub struct ShellApp {
    pub(super) model: ShellModel,
    pub(super) url_input: String,
    state_path: Option<std::path::PathBuf>,
    close_after_frames: Option<u32>,
    frames: u32,
}

impl ShellApp {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        model: ShellModel,
        close_after_frames: Option<u32>,
        state_path: Option<std::path::PathBuf>,
    ) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self {
            model,
            url_input: "https://example.com".to_owned(),
            state_path,
            close_after_frames,
            frames: 0,
        }
    }

    fn top_bar(&mut self, ui: &mut Ui) {
        if ui.button("New").clicked() {
            self.dispatch(AppCommand::NewScratchBuffer);
        }
        if ui.button("Save").clicked() {
            self.dispatch(AppCommand::SaveFocused);
        }
        if ui.button("Split R").clicked() {
            self.dispatch(AppCommand::SplitFocused(SplitDirection::Right));
        }
        if ui.button("Split D").clicked() {
            self.dispatch(AppCommand::SplitFocused(SplitDirection::Down));
        }
        if ui.button("Explorer").clicked() {
            self.dispatch(AppCommand::ToggleExplorer);
        }
        if ui.button("Markdown").clicked() {
            self.dispatch(AppCommand::ToggleMarkdownPreview);
        }
        if ui.button("Browser").clicked() {
            self.dispatch(AppCommand::OpenBrowser(self.url_input.clone()));
        }
        ui.text_edit_singleline(&mut self.url_input);
    }

    fn dispatch(&mut self, command: AppCommand) {
        self.model
            .dispatch(CommandEnvelope::new(CommandSource::Menu, command));
    }

    fn render_node(&mut self, ui: &mut Ui, node: TileNode) {
        match node {
            TileNode::Leaf { panes, active } => self.render_leaf(ui, panes, active),
            TileNode::Split {
                axis,
                ratio,
                first,
                second,
            } => self.render_split(ui, axis, ratio, *first, *second),
        }
    }

    fn render_split(
        &mut self,
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
                    self.render_node(ui, first);
                    ui.separator();
                    self.render_node(ui, second);
                });
            }
            SplitAxis::Horizontal => {
                ui.vertical(|ui| {
                    ui.set_height((available.y * ratio).max(160.0));
                    self.render_node(ui, first);
                    ui.separator();
                    self.render_node(ui, second);
                });
            }
        }
    }

    fn render_leaf(&mut self, ui: &mut Ui, panes: Vec<PaneId>, active: usize) {
        ui.group(|ui| {
            ui.horizontal_wrapped(|ui| {
                for pane_id in &panes {
                    if let Some(pane) = self.model.session.pane(*pane_id).cloned() {
                        let selected = self.model.session.focused_pane == *pane_id;
                        if ui.selectable_label(selected, pane.title).clicked() {
                            self.model.session.focus(*pane_id);
                        }
                    }
                }
                if ui.button("x").clicked() {
                    self.dispatch(AppCommand::CloseFocusedPane);
                }
            });
            let pane = panes
                .get(active)
                .and_then(|id| self.model.session.pane(*id))
                .cloned();
            match pane {
                Some(pane) => self.render_pane(ui, pane),
                None => {
                    ui.label("No pane");
                }
            };
        });
    }

    fn render_pane(&mut self, ui: &mut Ui, pane: PaneState) {
        match pane.kind {
            PaneKind::Editor => self.render_editor(ui, pane.binding),
            PaneKind::Explorer => self.render_explorer(ui),
            PaneKind::Markdown => self.render_markdown(ui),
            PaneKind::Browser => self.render_browser(ui),
            PaneKind::Settings => self.render_settings(ui),
        }
    }
}

impl eframe::App for ShellApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        self.frames += 1;
        self.persist();
        if self
            .close_after_frames
            .is_some_and(|limit| self.frames >= limit)
        {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            self.top_bar(ui);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let root = self.model.session.layout.root.clone();
            self.render_node(ui, root);
        });
    }
}

impl ShellApp {
    fn persist(&self) {
        if let Some(path) = &self.state_path {
            let _ = ShellSnapshot::from_shell(&self.model).save(path.clone());
        }
    }
}
