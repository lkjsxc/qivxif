mod chunk;
mod connect;
mod join;
mod mutate;

use anyhow::Result;
use clap::{Parser, Subcommand};
use qivxif_core::{BlockPos, ChunkCoord};
use tracing_subscriber::{EnvFilter, fmt};

#[derive(Parser)]
#[command(name = "qivxif-client-cli")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Connect {
        #[arg(long)]
        addr: String,
        #[arg(long, default_value = "client-cli")]
        player: String,
        #[arg(long, default_value_t = 0)]
        chunk_x: i32,
        #[arg(long, default_value_t = 0)]
        chunk_z: i32,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();
    match Cli::parse().command {
        Command::Connect {
            addr,
            player,
            chunk_x,
            chunk_z,
        } => connect_sequence(&addr, &player, ChunkCoord { x: chunk_x, z: chunk_z }).await?,
    }
    Ok(())
}

async fn connect_sequence(addr: &str, player: &str, coord: ChunkCoord) -> Result<()> {
    let client = connect::Client::connect(addr).await?;
    let hello = client.hello().await?;
    println!("hello ok: session={} world={}", hello.session_id, hello.world_epoch);
    join::join_world(&client, player).await?;
    println!("joined: {player}");
    let cells = chunk::request_chunk(&client, coord).await?;
    println!("chunk ({}, {}) cells={}", coord.x, coord.z, cells.len());
    let pos = BlockPos { x: coord.x * 16 + 1, y: 3, z: coord.z * 16 + 1 };
    mutate::place_block(&client, 1, pos, 9).await?;
    println!("placed block at {},{},{}", pos.x, pos.y, pos.z);
    mutate::flush_persistence(&client, 2).await?;
    println!("flush ok");
    Ok(())
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    fmt().with_env_filter(filter).with_target(false).init();
}
