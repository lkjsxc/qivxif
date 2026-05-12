//! Deterministic CPU frame evidence and native GPU frame submission for cached
//! authoritative block cells.

mod gpu;
mod scene;
mod smoke;

pub use gpu::GpuRenderer;
pub use scene::{CameraView, CellQuad, FrameStats, RenderScene};
pub use smoke::SmokeFrame;

pub(crate) fn shade(color: qivxif_assets::Color, y: i32) -> qivxif_assets::Color {
    let lift = y.clamp(0, 12) as u8 * 4;
    qivxif_assets::Color::new(
        color.r.saturating_add(lift),
        color.g.saturating_add(lift),
        color.b.saturating_add(lift),
    )
}
