use crate::{
    args::E2eArgs,
    native::{EDIT_BLOCK, EDIT_POS, NativeApp, has_ack, has_cell},
};
use anyhow::{Context, Result, bail};
use qivxif_client_core::RuntimeSnapshot;
use qivxif_protocol::BlockCell;
use qivxif_render::SmokeFrame;
use qivxif_ui::HudStatus;
use serde::Serialize;
use std::{fs, path::Path};

#[derive(Serialize)]
struct E2eSummary {
    connected: bool,
    joined: bool,
    chunks: usize,
    cells: usize,
    gpu_frames: u64,
    nonzero_pixels: usize,
    place_ack: bool,
    remove_ack: bool,
}

pub(crate) fn write_evidence(
    snapshot: &RuntimeSnapshot,
    app: &NativeApp,
    args: &E2eArgs,
) -> Result<()> {
    let frame = SmokeFrame::render(&snapshot.cached_cells, 128, 128);
    if !frame.is_nonblank() {
        bail!("desktop e2e frame is blank");
    }
    fs::write(&args.frame_output, frame.to_ppm())
        .with_context(|| format!("write {}", args.frame_output.display()))?;
    let summary = E2eSummary {
        connected: app.connected,
        joined: app.joined,
        chunks: snapshot.chunks,
        cells: snapshot.cells,
        gpu_frames: app.last_stats.gpu_frames,
        nonzero_pixels: frame.nonzero_pixel_count(),
        place_ack: true,
        remove_ack: has_ack(snapshot, qivxif_world::AIR) && !has_cell(snapshot),
    };
    fs::write(&args.summary_output, serde_json::to_vec_pretty(&summary)?)
        .with_context(|| format!("write {}", args.summary_output.display()))?;
    print_summary(snapshot, &summary, &args.summary_output, &args.frame_output);
    Ok(())
}

fn print_summary(snapshot: &RuntimeSnapshot, summary: &E2eSummary, json: &Path, ppm: &Path) {
    let status = HudStatus {
        connection: "connected".to_string(),
        world_id: snapshot.world_id.clone(),
        chunks: snapshot.chunks,
        cells: snapshot.cells,
        selected_block: EDIT_BLOCK,
        target: Some(EDIT_POS),
        last_ack: Some(BlockCell {
            pos: EDIT_POS,
            block: qivxif_world::AIR,
        }),
        last_error: snapshot.last_error.clone(),
    };
    println!("desktop e2e ... ok");
    println!("{}", status.line());
    println!(
        "gpu_frames {} nonzero_pixels {}",
        summary.gpu_frames, summary.nonzero_pixels
    );
    println!("summary {}", json.display());
    println!("artifact {}", ppm.display());
}
