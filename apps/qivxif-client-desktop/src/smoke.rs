use crate::args::SmokeFrameArgs;
use anyhow::{Result, bail};
use qivxif_client_core::{ClientConfig, ClientRuntime};
use qivxif_core::ChunkCoord;
use qivxif_render::SmokeFrame;
use qivxif_ui::HudStatus;
use std::fs;

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
    let cells = runtime.cache().cells();
    let frame = SmokeFrame::render(&cells, args.size, args.size);
    if !frame.is_nonblank() {
        bail!("desktop smoke frame is blank");
    }
    fs::write(&args.output, frame.to_ppm())?;
    let summary = runtime.summary();
    let status = HudStatus {
        connection: "connected".to_string(),
        world_id: summary.world_id,
        chunks: summary.chunks,
        cells: summary.cells,
    };
    println!("desktop smoke ... ok");
    println!("{}", status.line());
    println!("artifact {}", args.output.display());
    Ok(())
}
