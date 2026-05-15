use crate::{BrowserBounds, BrowserPolicy, BrowserState};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum BrowserError {
    #[error("url is empty")]
    EmptyUrl,
    #[error("browser backend is unavailable")]
    BackendUnavailable,
}

pub trait BrowserController {
    fn state(&self) -> &BrowserState;
    fn policy(&self) -> &BrowserPolicy;
    fn navigate(&mut self, url: &str) -> Result<(), BrowserError>;
    fn set_bounds(&mut self, bounds: BrowserBounds);
    fn open_external(&self, url: &str) -> Result<(), BrowserError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PermissionDecision;
    use std::path::PathBuf;

    struct MemoryBrowser {
        state: BrowserState,
        policy: BrowserPolicy,
        bounds: BrowserBounds,
    }

    impl BrowserController for MemoryBrowser {
        fn state(&self) -> &BrowserState {
            &self.state
        }

        fn policy(&self) -> &BrowserPolicy {
            &self.policy
        }

        fn navigate(&mut self, url: &str) -> Result<(), BrowserError> {
            if url.is_empty() {
                return Err(BrowserError::EmptyUrl);
            }
            self.state.current_url = url.to_owned();
            Ok(())
        }

        fn set_bounds(&mut self, bounds: BrowserBounds) {
            self.bounds = bounds;
        }

        fn open_external(&self, url: &str) -> Result<(), BrowserError> {
            (!url.is_empty())
                .then_some(())
                .ok_or(BrowserError::EmptyUrl)
        }
    }

    #[test]
    fn controller_contract_updates_navigation() {
        let policy = BrowserPolicy::locked_down(PathBuf::from("downloads"));
        let mut browser = MemoryBrowser {
            state: BrowserState::new("about:blank"),
            policy,
            bounds: BrowserBounds {
                x: 0,
                y: 0,
                width: 100,
                height: 80,
            },
        };
        browser.navigate("https://example.com").unwrap();
        assert_eq!(browser.state().current_url, "https://example.com");
        assert_eq!(browser.policy().camera, PermissionDecision::Deny);
    }
}
