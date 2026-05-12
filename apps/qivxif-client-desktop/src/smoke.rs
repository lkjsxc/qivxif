use crate::args::SmokeFrameArgs;
use anyhow::{Result, bail};
use qivxif_client_core::{ClientConfig, ClientRuntime};
use qivxif_core::{BlockPos, ChunkCoord};
use qivxif_protocol::BlockCell;
use qivxif_render::SmokeFrame;
use qivxif_ui::HudStatus;
use std::fs;

const SMOKE_EDIT_POS: BlockPos = BlockPos { x: 1, y: 3, z: 1 };
const SMOKE_EDIT_BLOCK: u16 = 9;

pub async fn run(args: SmokeFrameArgs) -> Result<()> {
    let config = ClientConfig {
        addr: args.addr,
        server_name: args.server_name,
        tls_mode: args.tls,
    };
    let mut runtime = ClientRuntime::connect_join(&config, &args.player).await?;
    runtime
        .fetch_neighborhood(ChunkCoord { x: 0, z: 0 }, args.radius)
        .await?;
    runtime
        .place_block(SMOKE_EDIT_POS, SMOKE_EDIT_BLOCK)
        .await?;
    let smoke_cell = BlockCell {
        pos: SMOKE_EDIT_POS,
        block: SMOKE_EDIT_BLOCK,
    };
    if !runtime.cache().contains_cell(&smoke_cell) {
        bail!("desktop smoke mutation ack missing from cache: {smoke_cell:?}");
    }
    let cells = runtime.cache().cells();
    let frame = SmokeFrame::render(&cells, args.size, args.size);
    if !frame.is_nonblank() {
        bail!("desktop smoke frame is blank");
    }
    let ppm = frame.to_ppm();
    let byte_count = ppm.len();
    let nonzero_pixels = frame.nonzero_pixel_count();
    fs::write(&args.output, ppm)?;
    let summary = runtime.summary();
    let status = HudStatus {
        connection: "connected".to_string(),
        world_id: summary.world_id,
        chunks: summary.chunks,
        cells: summary.cells,
    };
    println!("desktop smoke ... ok");
    println!("{}", status.line());
    println!("frame bytes {byte_count} nonzero_pixels {nonzero_pixels}");
    println!("artifact {}", args.output.display());
    Ok(())
}
