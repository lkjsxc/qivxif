use super::ShellApp;
use eframe::egui::{RichText, ScrollArea, TextEdit, Ui};
use qivxif_markdown::MarkdownBlock;
use qivxif_workspace::PaneBinding;

impl ShellApp {
    pub(super) fn render_editor(&mut self, ui: &mut Ui, binding: PaneBinding) {
        let PaneBinding::Editor { buffer_id } = binding else {
            ui.label("Pane is not bound to a buffer");
            return;
        };
        let Some(index) = self
            .model
            .buffers
            .iter()
            .position(|buffer| buffer.id() == buffer_id)
        else {
            ui.label("Missing buffer");
            return;
        };
        let mut text = self.model.buffers[index].text();
        let response = ui.add_sized(
            ui.available_size(),
            TextEdit::multiline(&mut text)
                .code_editor()
                .desired_width(f32::INFINITY)
                .desired_rows(24),
        );
        if response.changed() {
            self.model.buffers[index].restore_text(text);
            if let Some(state) = self.model.session.buffer_mut(buffer_id) {
                state.dirty = true;
            }
        }
    }

    pub(super) fn render_explorer(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui.button("Refresh").clicked() {
                let _ = self.model.explorer.refresh_root(0);
            }
            ui.checkbox(&mut self.model.explorer.show_hidden, "Hidden");
        });
        ScrollArea::vertical().show(ui, |ui| {
            for entry in self.model.explorer.entries.clone() {
                let prefix = if entry.is_dir { "[dir]" } else { "[file]" };
                if ui.button(format!("{prefix} {}", entry.name)).clicked() && !entry.is_dir {
                    let _ = self.model.open_file(entry.path);
                }
            }
        });
    }

    pub(super) fn render_markdown(&mut self, ui: &mut Ui) {
        if ui.button("Render focused buffer").clicked()
            && let Some(id) = self.model.session.focused_editor_buffer()
            && let Some(buffer) = self.model.buffers.iter().find(|buffer| buffer.id() == id)
        {
            self.model.render_markdown(&buffer.text());
        }
        ScrollArea::vertical().show(ui, |ui| {
            for block in &self.model.markdown.blocks {
                match block {
                    MarkdownBlock::Heading { text, .. } => {
                        ui.heading(text);
                    }
                    MarkdownBlock::Paragraph(text) => {
                        ui.label(text);
                    }
                    MarkdownBlock::CodeBlock { language, code } => {
                        ui.label(RichText::new(language.as_deref().unwrap_or("code")).strong());
                        ui.monospace(code);
                    }
                    MarkdownBlock::Rule => {
                        ui.separator();
                    }
                }
            }
        });
    }

    pub(super) fn render_browser(&mut self, ui: &mut Ui) {
        ui.label("Browser policy fallback");
        ui.horizontal(|ui| {
            ui.label("URL");
            ui.text_edit_singleline(&mut self.url_input);
            if ui.button("Navigate").clicked() {
                self.model.browser_state =
                    Some(qivxif_browser::BrowserState::new(self.url_input.clone()));
            }
        });
        if let Some(state) = &self.model.browser_state {
            ui.monospace(&state.current_url);
        }
        ui.label("Camera, microphone, geolocation, and notifications are denied by default.");
    }

    pub(super) fn render_settings(&mut self, ui: &mut Ui) {
        ui.heading("Settings");
        ui.label("Theme: dark");
        ui.label("Font size: 14");
    }
}
