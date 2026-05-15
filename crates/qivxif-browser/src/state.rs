use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserState {
    pub current_url: String,
    pub title: String,
    pub loading: bool,
}

impl BrowserState {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            current_url: url.into(),
            title: String::new(),
            loading: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserBounds {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}
