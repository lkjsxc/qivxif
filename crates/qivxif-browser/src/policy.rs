use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermissionDecision {
    Deny,
    Ask,
    Allow,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserPolicy {
    pub camera: PermissionDecision,
    pub microphone: PermissionDecision,
    pub geolocation: PermissionDecision,
    pub notifications: PermissionDecision,
    pub downloads_dir: PathBuf,
    pub external_links_in_system_browser: bool,
}

impl BrowserPolicy {
    pub fn locked_down(downloads_dir: PathBuf) -> Self {
        Self {
            camera: PermissionDecision::Deny,
            microphone: PermissionDecision::Deny,
            geolocation: PermissionDecision::Deny,
            notifications: PermissionDecision::Deny,
            downloads_dir,
            external_links_in_system_browser: true,
        }
    }

    pub fn can_open_permission_prompt(&self) -> bool {
        [
            &self.camera,
            &self.microphone,
            &self.geolocation,
            &self.notifications,
        ]
        .contains(&&PermissionDecision::Ask)
    }
}
