use crate::{ShellEffect, ShellEvent, ShellModel};

impl ShellModel {
    pub(crate) fn execute_effect(&mut self, effect: ShellEffect) {
        match effect {
            ShellEffect::LoadFile(path) => {
                if let Err(error) = self.open_file(path) {
                    self.events.push(ShellEvent::Error(error.to_string()));
                }
            }
            ShellEffect::SaveFile { .. } => match self.save_focused() {
                Ok(()) => self.events.push(ShellEvent::SavedFocused),
                Err(error) => self.events.push(ShellEvent::Error(error.to_string())),
            },
            ShellEffect::RefreshExplorer(index) => {
                if let Err(error) = self.explorer.refresh_root(index) {
                    self.events.push(ShellEvent::Error(error.to_string()));
                }
            }
            ShellEffect::OpenExternalUrl(url) => {
                self.events.push(ShellEvent::BrowserOpened(url));
            }
            ShellEffect::RenderMarkdown(id) => {
                if let Some(buffer) = self.buffers.iter().find(|buffer| buffer.id() == id) {
                    let text = buffer.text();
                    self.render_markdown(&text);
                }
            }
            ShellEffect::PersistWorkspace
            | ShellEffect::PersistSettings
            | ShellEffect::WriteRecovery(_)
            | ShellEffect::ClearRecovery(_) => {}
        }
    }
}
