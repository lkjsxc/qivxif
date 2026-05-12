#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HudStatus {
    pub connection: String,
    pub world_id: String,
    pub chunks: usize,
    pub cells: usize,
}

impl HudStatus {
    pub fn line(&self) -> String {
        format!(
            "{} | world {} | chunks {} | cells {}",
            self.connection, self.world_id, self.chunks, self.cells
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
        };
        assert!(status.line().contains("chunks 9"));
    }
}
