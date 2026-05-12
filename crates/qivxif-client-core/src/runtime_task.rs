use crate::{
    ClientCommand, ClientConfig, ClientRuntime, RuntimeCommand, RuntimeEvent, RuntimeSnapshot,
    RuntimeStatus,
};
use qivxif_core::ChunkCoord;
use qivxif_protocol::BlockCell;
use tokio::sync::{mpsc, watch};

pub async fn runtime_task(
    config: ClientConfig,
    player: String,
    initial_radius: i32,
    mut commands: mpsc::UnboundedReceiver<RuntimeCommand>,
    snapshots: watch::Sender<RuntimeSnapshot>,
    events: mpsc::UnboundedSender<RuntimeEvent>,
) {
    let mut last_ack = None;
    let mut runtime = match ClientRuntime::connect_join(&config, &player).await {
        Ok(mut runtime) => {
            emit_events(&mut runtime, &events);
            send_snapshot(
                &runtime,
                &player,
                RuntimeStatus::Joined,
                None,
                None,
                &snapshots,
            );
            runtime
        }
        Err(error) => {
            fail(error.to_string(), &player, &snapshots, &events);
            return;
        }
    };
    if let Err(error) = runtime
        .fetch_neighborhood(ChunkCoord { x: 0, z: 0 }, initial_radius)
        .await
    {
        fail(error.to_string(), &player, &snapshots, &events);
        return;
    }
    emit_events(&mut runtime, &events);
    send_snapshot(
        &runtime,
        &player,
        RuntimeStatus::Ready,
        None,
        None,
        &snapshots,
    );
    command_loop(
        runtime,
        player,
        &mut commands,
        &snapshots,
        &events,
        &mut last_ack,
    )
    .await;
}

async fn command_loop(
    mut runtime: ClientRuntime,
    player: String,
    commands: &mut mpsc::UnboundedReceiver<RuntimeCommand>,
    snapshots: &watch::Sender<RuntimeSnapshot>,
    events: &mpsc::UnboundedSender<RuntimeEvent>,
    last_ack: &mut Option<BlockCell>,
) {
    while let Some(command) = commands.recv().await {
        if command == RuntimeCommand::Stop {
            let _ = events.send(RuntimeEvent::RuntimeStopped);
            send_snapshot(
                &runtime,
                &player,
                RuntimeStatus::Stopped,
                last_ack.clone(),
                None,
                snapshots,
            );
            return;
        }
        match apply_runtime_command(&mut runtime, command).await {
            Ok(ack) => {
                *last_ack = ack;
                emit_events(&mut runtime, events);
                send_snapshot(
                    &runtime,
                    &player,
                    RuntimeStatus::Ready,
                    last_ack.clone(),
                    None,
                    snapshots,
                );
            }
            Err(error) => {
                fail(error.to_string(), &player, snapshots, events);
                return;
            }
        }
    }
}

async fn apply_runtime_command(
    runtime: &mut ClientRuntime,
    command: RuntimeCommand,
) -> anyhow::Result<Option<BlockCell>> {
    match command {
        RuntimeCommand::FetchNeighborhood { center, radius } => {
            runtime.fetch_neighborhood(center, radius).await?;
            Ok(None)
        }
        RuntimeCommand::Place { pos, block } => {
            runtime
                .apply_command(ClientCommand::Place { pos, block })
                .await?;
            Ok(runtime
                .cache()
                .cells()
                .into_iter()
                .find(|cell| cell.pos == pos))
        }
        RuntimeCommand::Remove { pos } => {
            runtime.apply_command(ClientCommand::Remove { pos }).await?;
            Ok(Some(BlockCell {
                pos,
                block: qivxif_world::AIR,
            }))
        }
        RuntimeCommand::Stop => Ok(None),
    }
}

fn emit_events(runtime: &mut ClientRuntime, events: &mpsc::UnboundedSender<RuntimeEvent>) {
    for event in runtime.drain_events() {
        let _ = events.send(event);
    }
}

fn send_snapshot(
    runtime: &ClientRuntime,
    player: &str,
    status: RuntimeStatus,
    last_ack: Option<BlockCell>,
    last_error: Option<String>,
    snapshots: &watch::Sender<RuntimeSnapshot>,
) {
    let _ = snapshots.send(RuntimeSnapshot::from_runtime(
        runtime, player, status, last_ack, last_error,
    ));
}

fn fail(
    message: String,
    player: &str,
    snapshots: &watch::Sender<RuntimeSnapshot>,
    events: &mpsc::UnboundedSender<RuntimeEvent>,
) {
    let mut snapshot = RuntimeSnapshot::initial(player.to_string());
    snapshot.status = RuntimeStatus::Failed;
    snapshot.last_error = Some(message.clone());
    let _ = snapshots.send(snapshot);
    let _ = events.send(RuntimeEvent::RuntimeError { message });
}
