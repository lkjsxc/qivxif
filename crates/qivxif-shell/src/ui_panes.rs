use super::ShellApp;
use eframe::egui::{RichText, ScrollArea, TextEdit, Ui};
use qivxif_action::AppCommand;
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
            self.dispatch(AppCommand::EditBuffer {
                buffer_id: buffer_id.raw(),
                text,
            });
        }
    }

    pub(super) fn render_explorer(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui.button("Refresh").clicked() {
                self.dispatch(AppCommand::RefreshExplorer);
            }
            if ui
                .checkbox(&mut self.model.explorer.show_hidden, "Hidden")
                .changed()
            {
                self.dispatch(AppCommand::RefreshExplorer);
            }
        });
        if let Some(error) = &self.model.explorer.error {
            ui.label(RichText::new(error).color(eframe::egui::Color32::from_rgb(255, 123, 114)));
        }
        ScrollArea::vertical().show(ui, |ui| {
            for entry in self.model.explorer.entries.clone() {
                let prefix = if entry.is_dir { "[dir]" } else { "[file]" };
                if ui.button(format!("{prefix} {}", entry.name)).clicked() && !entry.is_dir {
                    self.dispatch(AppCommand::OpenPath(entry.path));
                }
            }
        });
    }

    pub(super) fn render_markdown(&mut self, ui: &mut Ui) {
        if let Some(id) = self.model.session.focused_editor_buffer()
            && let Some(buffer) = self.model.buffers.iter().find(|buffer| buffer.id() == id)
            && self.model.markdown.source_revision != Some(buffer.revision())
        {
            let text = buffer.text();
            let revision = buffer.revision();
            self.model.render_markdown(&text);
            self.model.markdown.source_revision = Some(revision);
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
                    MarkdownBlock::ListItem { text, checked } => {
                        let marker = match checked {
                            Some(true) => "[x]",
                            Some(false) => "[ ]",
                            None => "-",
                        };
                        ui.label(format!("{marker} {text}"));
                    }
                    MarkdownBlock::TableRow(cells) => {
                        ui.horizontal_wrapped(|ui| {
                            for cell in cells {
                                ui.label(cell);
                            }
                        });
                    }
                }
            }
        });
    }

    pub(super) fn render_browser(&mut self, ui: &mut Ui) {
        ui.label("Browser fallback");
        ui.horizontal(|ui| {
            ui.label("URL");
            ui.text_edit_singleline(&mut self.url_input);
            if ui.button("Navigate").clicked() {
                self.dispatch(AppCommand::OpenBrowser(self.url_input.clone()));
            }
            if ui.button("Back").clicked() {
                self.dispatch(AppCommand::BrowserBack);
            }
            if ui.button("Forward").clicked() {
                self.dispatch(AppCommand::BrowserForward);
            }
            if ui.button("Open external").clicked() {
                self.dispatch(AppCommand::BrowserOpenExternal);
            }
        });
        if let Some(state) = &self.model.browser_state {
            ui.monospace(&state.current_url);
            if let Some(error) = &state.last_error {
                ui.label(
                    RichText::new(error).color(eframe::egui::Color32::from_rgb(255, 123, 114)),
                );
            }
        }
        ui.label("Camera, microphone, geolocation, and notifications are denied by default.");
    }

    pub(super) fn render_settings(&mut self, ui: &mut Ui) {
        ui.heading("Settings");
        ui.label("Theme: dark");
        let mut size = self.model.session.settings.font_size;
        if ui
            .add(eframe::egui::Slider::new(&mut size, 8..=32).text("Font size"))
            .changed()
        {
            self.dispatch(AppCommand::SetFontSize(size));
        }
        if ui.button("Save settings").clicked() {
            self.dispatch(AppCommand::PersistSettings);
        }
    }
}
