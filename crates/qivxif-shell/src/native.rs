use crate::{ShellModel, ui::ShellApp};
use eframe::{NativeOptions, egui};
use qivxif_browser::BrowserPolicy;
use std::{error::Error, path::PathBuf};

#[derive(Debug, Clone)]
pub struct NativeRunConfig {
    pub title: String,
    pub initial_window_size: [f32; 2],
    pub close_after_frames: Option<u32>,
    pub state_path: Option<PathBuf>,
}

impl Default for NativeRunConfig {
    fn default() -> Self {
        Self {
            title: "qivxif".to_owned(),
            initial_window_size: [1200.0, 780.0],
            close_after_frames: None,
            state_path: None,
        }
    }
}

pub fn run_native() -> Result<(), Box<dyn Error>> {
    let policy = BrowserPolicy::locked_down(PathBuf::from("downloads"));
    run_native_with_model(ShellModel::new(policy), NativeRunConfig::default())
}

pub fn run_native_with_model(
    model: ShellModel,
    config: NativeRunConfig,
) -> Result<(), Box<dyn Error>> {
    let viewport = egui::ViewportBuilder::default()
        .with_title(config.title.clone())
        .with_inner_size(config.initial_window_size);
    let options = NativeOptions {
        viewport,
        renderer: eframe::Renderer::Wgpu,
        ..NativeOptions::default()
    };
    eframe::run_native(
        &config.title,
        options,
        Box::new(|cc| {
            Ok(Box::new(ShellApp::new(
                cc,
                model,
                config.close_after_frames,
                config.state_path,
            )))
        }),
    )?;
    Ok(())
}
