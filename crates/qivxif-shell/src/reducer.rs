use crate::{ShellEvent, ShellModel};
use qivxif_action::{AppCommand, CommandEnvelope, PanePlacement, PaneSpawn};
use qivxif_browser::BrowserState;
use qivxif_editor_buffer::{BufferId, TextEdit, TextRange};
use qivxif_tiles::{PaneId, PaneKind};
use qivxif_workspace::{PaneBinding, PaneState};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ShellTransition {
    pub state: ShellModel,
    pub effects: Vec<ShellEffect>,
    pub events: Vec<ShellEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShellEffect {
    LoadFile(PathBuf),
    SaveFile { buffer_id: BufferId, path: PathBuf },
    RefreshExplorer(usize),
    PersistWorkspace,
    PersistSettings,
    OpenExternalUrl(String),
    WriteRecovery(BufferId),
    ClearRecovery(BufferId),
    RenderMarkdown(BufferId),
}

pub fn reduce_shell(shell: &ShellModel, envelope: CommandEnvelope) -> ShellTransition {
    let mut next = shell.clone();
    let mut effects = Vec::new();
    let mut events = Vec::new();
    match envelope.command {
        AppCommand::NewScratchBuffer => next.new_scratch("scratch"),
        AppCommand::OpenPath(path) => effects.push(ShellEffect::LoadFile(path)),
        AppCommand::SaveFocused => {
            if let Some((buffer_id, path)) = focused_save_target(&next) {
                effects.push(ShellEffect::SaveFile { buffer_id, path });
            } else {
                events.push(ShellEvent::Error("focused buffer has no path".into()));
            }
        }
        AppCommand::EditBuffer { buffer_id, text } => {
            edit_buffer(&mut next, BufferId::from_raw(buffer_id), text, &mut effects)
        }
        AppCommand::UndoFocused => next.undo_focused(),
        AppCommand::RedoFocused => next.redo_focused(),
        AppCommand::CloseFocusedPane => {
            let focused = next.session.focused_pane;
            next.session.close(focused);
        }
        AppCommand::ClosePane(raw) => {
            next.session.close(PaneId::from_raw(raw));
        }
        AppCommand::SplitFocused(direction) => next.split_focused(direction),
        AppCommand::TabFocused(spawn) => spawn_pane(&mut next, spawn, PanePlacement::TabFocused),
        AppCommand::SpawnPane(spawn, placement) => spawn_pane(&mut next, spawn, placement),
        AppCommand::FocusNextPane => next.focus_next(),
        AppCommand::FocusPane(raw) => {
            next.session.focus(PaneId::from_raw(raw));
        }
        AppCommand::ToggleMaximize => next.session.toggle_maximize(),
        AppCommand::ToggleExplorer => {
            spawn_pane(&mut next, PaneSpawn::Explorer, PanePlacement::SplitRight)
        }
        AppCommand::ToggleMarkdownPreview => {
            let source = next.session.focused_editor_buffer().map(|id| id.raw());
            spawn_pane(
                &mut next,
                PaneSpawn::Markdown {
                    source_buffer_id: source,
                },
                PanePlacement::SplitRight,
            );
        }
        AppCommand::OpenBrowser(url) => {
            let browser = next
                .browser_state
                .get_or_insert_with(|| BrowserState::new("about:blank"));
            match browser.navigate(&url) {
                Ok(()) => events.push(ShellEvent::BrowserOpened(browser.current_url.clone())),
                Err(error) => {
                    browser.last_error = Some(error.clone());
                    events.push(ShellEvent::Error(error));
                }
            }
        }
        AppCommand::BrowserBack => {
            if let Some(browser) = &mut next.browser_state {
                browser.back();
            }
        }
        AppCommand::BrowserForward => {
            if let Some(browser) = &mut next.browser_state {
                browser.forward();
            }
        }
        AppCommand::BrowserReload => {}
        AppCommand::BrowserOpenExternal => {
            if let Some(browser) = &next.browser_state {
                effects.push(ShellEffect::OpenExternalUrl(browser.current_url.clone()));
            }
        }
        AppCommand::ToggleHiddenFiles => next.explorer.show_hidden = !next.explorer.show_hidden,
        AppCommand::RefreshExplorer => effects.push(ShellEffect::RefreshExplorer(0)),
        AppCommand::PersistSettings => effects.push(ShellEffect::PersistSettings),
        AppCommand::SetFontSize(size) => {
            next.session.settings.font_size = size.clamp(8, 32);
            effects.push(ShellEffect::PersistSettings);
        }
    }
    effects.push(ShellEffect::PersistWorkspace);
    ShellTransition {
        state: next,
        effects,
        events,
    }
}

fn focused_save_target(shell: &ShellModel) -> Option<(BufferId, PathBuf)> {
    let id = shell.session.focused_editor_buffer()?;
    let path = shell
        .session
        .buffers
        .iter()
        .find(|state| state.id == id)?
        .path
        .clone()?;
    Some((id, path))
}

fn edit_buffer(shell: &mut ShellModel, id: BufferId, text: String, effects: &mut Vec<ShellEffect>) {
    let Some(buffer) = shell.buffers.iter_mut().find(|buffer| buffer.id() == id) else {
        return;
    };
    let old = buffer.text();
    if old == text {
        return;
    }
    if let Ok(batch) = buffer.apply(TextEdit::replace(TextRange::new(0, old.len()), text)) {
        if let Some(history) = shell.histories.iter_mut().find(|item| item.buffer_id == id) {
            history.history.record(batch);
        }
        if let Some(state) = shell.session.buffer_mut(id) {
            state.dirty = true;
        }
        effects.push(ShellEffect::WriteRecovery(id));
    }
}

fn spawn_pane(shell: &mut ShellModel, spawn: PaneSpawn, placement: PanePlacement) {
    let pane = match spawn {
        PaneSpawn::Editor { buffer_id } => {
            let id = buffer_id
                .map(BufferId::from_raw)
                .or_else(|| shell.session.focused_editor_buffer());
            id.map(|id| PaneState::editor(id, "Editor"))
        }
        PaneSpawn::Explorer => Some(PaneState::new(
            PaneKind::Explorer,
            "Explorer",
            PaneBinding::Explorer {
                root: shell.explorer.roots.first().cloned(),
            },
        )),
        PaneSpawn::Markdown { source_buffer_id } => Some(PaneState::new(
            PaneKind::Markdown,
            "Markdown",
            PaneBinding::Markdown {
                source_buffer_id: source_buffer_id.map(BufferId::from_raw),
            },
        )),
        PaneSpawn::Browser { url } => {
            if let Some(url) = url {
                shell.browser_state = Some(BrowserState::new(url));
            }
            Some(PaneState::new(
                PaneKind::Browser,
                "Browser",
                PaneBinding::Browser { browser_id: 1 },
            ))
        }
        PaneSpawn::Settings => Some(PaneState::new(
            PaneKind::Settings,
            "Settings",
            PaneBinding::Settings,
        )),
    };
    if let Some(pane) = pane {
        shell.place_pane(pane, placement);
    }
}
