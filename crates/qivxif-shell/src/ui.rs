use crate::ShellModel;
use eframe::egui::{self, Ui};
use qivxif_tiles::PaneKind;
use qivxif_workspace::PaneState;

#[path = "ui_chrome.rs"]
mod ui_chrome;
#[path = "ui_layout.rs"]
mod ui_layout;
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

    pub(super) fn render_pane(&mut self, ui: &mut Ui, pane: PaneState) {
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
            ui_layout::render_workspace(self, ui);
        });
    }
}
