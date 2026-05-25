use super::ShellApp;
use crate::ShellSnapshot;
use eframe::egui::Ui;
use qivxif_action::{
    AppCommand, CommandEnvelope, CommandSource, PanePlacement, PaneSpawn, SplitDirection,
};

impl ShellApp {
    pub(super) fn top_bar(&mut self, ui: &mut Ui) {
        if ui.button("New").clicked() {
            self.dispatch(AppCommand::NewScratchBuffer);
        }
        if ui.button("Save").clicked() {
            self.dispatch(AppCommand::SaveFocused);
        }
        if ui.button("Undo").clicked() {
            self.dispatch(AppCommand::UndoFocused);
        }
        if ui.button("Redo").clicked() {
            self.dispatch(AppCommand::RedoFocused);
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
            self.dispatch(AppCommand::SpawnPane(
                PaneSpawn::Browser {
                    url: Some(self.url_input.clone()),
                },
                PanePlacement::SplitRight,
            ));
        }
        if ui.button("Settings").clicked() {
            self.dispatch(AppCommand::SpawnPane(
                PaneSpawn::Settings,
                PanePlacement::TabFocused,
            ));
        }
        if ui.button("Max").clicked() {
            self.dispatch(AppCommand::ToggleMaximize);
        }
        ui.text_edit_singleline(&mut self.url_input);
        if let Some(path) = &self.state_path {
            ui.monospace(path.display().to_string());
        }
    }

    pub(super) fn dispatch(&mut self, command: AppCommand) {
        self.model
            .dispatch(CommandEnvelope::new(CommandSource::Menu, command));
    }

    pub(super) fn persist(&self) {
        if let Some(path) = &self.state_path {
            let _ = ShellSnapshot::from_shell(&self.model).save(path.clone());
        }
    }
}
