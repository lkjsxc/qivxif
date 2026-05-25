use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,
    pub font_size: u16,
    pub autosave_recovery: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "dark".to_owned(),
            font_size: 14,
            autosave_recovery: true,
        }
    }
}
