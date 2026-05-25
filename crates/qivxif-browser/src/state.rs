use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserState {
    pub current_url: String,
    pub title: String,
    pub loading: bool,
    pub back_stack: Vec<String>,
    pub forward_stack: Vec<String>,
    pub last_error: Option<String>,
}

impl BrowserState {
    pub fn new(url: impl Into<String>) -> Self {
        let current_url = normalize_url(&url.into()).unwrap_or_else(|_| "about:blank".to_owned());
        Self {
            current_url,
            title: String::new(),
            loading: false,
            back_stack: Vec::new(),
            forward_stack: Vec::new(),
            last_error: None,
        }
    }

    pub fn navigate(&mut self, url: &str) -> Result<(), String> {
        let url = normalize_url(url)?;
        if self.current_url != url {
            self.back_stack.push(self.current_url.clone());
            self.current_url = url;
            self.forward_stack.clear();
        }
        self.loading = false;
        self.last_error = None;
        Ok(())
    }

    pub fn back(&mut self) -> bool {
        let Some(previous) = self.back_stack.pop() else {
            return false;
        };
        self.forward_stack.push(self.current_url.clone());
        self.current_url = previous;
        true
    }

    pub fn forward(&mut self) -> bool {
        let Some(next) = self.forward_stack.pop() else {
            return false;
        };
        self.back_stack.push(self.current_url.clone());
        self.current_url = next;
        true
    }
}

pub fn normalize_url(input: &str) -> Result<String, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("url is empty".to_owned());
    }
    if trimmed == "about:blank" {
        return Ok(trimmed.to_owned());
    }
    let candidate = if trimmed.contains("://") {
        trimmed.to_owned()
    } else {
        format!("https://{trimmed}")
    };
    let Some((scheme, rest)) = candidate.split_once("://") else {
        return Err("url is missing scheme".to_owned());
    };
    if !matches!(scheme, "http" | "https") || rest.trim_matches('/').is_empty() {
        return Err("only http and https urls are supported".to_owned());
    }
    Ok(candidate)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserBounds {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn navigation_normalizes_and_tracks_history() {
        let mut state = BrowserState::new("example.com");
        assert_eq!(state.current_url, "https://example.com");
        state.navigate("https://example.org").unwrap();
        assert!(state.back());
        assert_eq!(state.current_url, "https://example.com");
        assert!(state.forward());
        assert_eq!(state.current_url, "https://example.org");
        assert!(normalize_url("file:///tmp/a").is_err());
    }
}
