//! Shared HUD state used by smoke output and native client overlays.

use qivxif_core::BlockPos;
use qivxif_protocol::BlockCell;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HudStatus {
    pub connection: String,
    pub world_id: String,
    pub chunks: usize,
    pub cells: usize,
    pub selected_block: u16,
    pub target: Option<BlockPos>,
    pub last_ack: Option<BlockCell>,
    pub last_error: Option<String>,
}

impl HudStatus {
    pub fn line(&self) -> String {
        let target = self
            .target
            .map(|pos| format!("{},{},{}", pos.x, pos.y, pos.z))
            .unwrap_or_else(|| "none".to_string());
        let ack = self
            .last_ack
            .as_ref()
            .map(|cell| {
                format!(
                    "{}@{},{},{}",
                    cell.block, cell.pos.x, cell.pos.y, cell.pos.z
                )
            })
            .unwrap_or_else(|| "none".to_string());
        let error = self.last_error.as_deref().unwrap_or("none");
        format!(
            "{} | world {} | chunks {} | cells {} | selected {} | target {} | ack {} | error {}",
            self.connection,
            self.world_id,
            self.chunks,
            self.cells,
            self.selected_block,
            target,
            ack,
            error
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_line_is_compact() {
        let status = HudStatus {
            connection: "connected".to_string(),
            world_id: "world-1".to_string(),
            chunks: 9,
            cells: 64,
            selected_block: 9,
            target: None,
            last_ack: None,
            last_error: None,
        };
        assert!(status.line().contains("chunks 9"));
    }
}
