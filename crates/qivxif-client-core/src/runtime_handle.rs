use crate::{ClientConfig, ClientRuntime, RuntimeEvent, runtime_task::runtime_task};
use qivxif_core::{BlockPos, ChunkCoord};
use qivxif_protocol::BlockCell;
use tokio::{
    runtime::Handle,
    sync::{mpsc, watch},
    task::JoinHandle,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeStatus {
    Connecting,
    Connected,
    Joined,
    Ready,
    Stopped,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeSnapshot {
    pub status: RuntimeStatus,
    pub world_id: String,
    pub player: String,
    pub chunks: usize,
    pub cells: usize,
    pub cached_cells: Vec<BlockCell>,
    pub last_ack: Option<BlockCell>,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeCommand {
    FetchNeighborhood { center: ChunkCoord, radius: i32 },
    Place { pos: BlockPos, block: u16 },
    Remove { pos: BlockPos },
    Stop,
}

pub struct ClientRuntimeHandle {
    commands: mpsc::UnboundedSender<RuntimeCommand>,
    snapshots: watch::Receiver<RuntimeSnapshot>,
    events: mpsc::UnboundedReceiver<RuntimeEvent>,
    _task: JoinHandle<()>,
}

impl RuntimeSnapshot {
    pub fn initial(player: impl Into<String>) -> Self {
        Self {
            status: RuntimeStatus::Connecting,
            world_id: String::new(),
            player: player.into(),
            chunks: 0,
            cells: 0,
            cached_cells: Vec::new(),
            last_ack: None,
            last_error: None,
        }
    }

    pub(crate) fn from_runtime(
        runtime: &ClientRuntime,
        player: &str,
        status: RuntimeStatus,
        last_ack: Option<BlockCell>,
        last_error: Option<String>,
    ) -> Self {
        let summary = runtime.summary();
        Self {
            status,
            world_id: summary.world_id,
            player: player.to_string(),
            chunks: summary.chunks,
            cells: summary.cells,
            cached_cells: runtime.cache().cells(),
            last_ack,
            last_error,
        }
    }
}

impl ClientRuntimeHandle {
    pub fn spawn(
        handle: Handle,
        config: ClientConfig,
        player: String,
        initial_radius: i32,
    ) -> Self {
        let (command_tx, command_rx) = mpsc::unbounded_channel();
        let (snapshot_tx, snapshot_rx) = watch::channel(RuntimeSnapshot::initial(player.clone()));
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let task = handle.spawn(runtime_task(
            config,
            player,
            initial_radius,
            command_rx,
            snapshot_tx,
            event_tx,
        ));
        Self {
            commands: command_tx,
            snapshots: snapshot_rx,
            events: event_rx,
            _task: task,
        }
    }

    pub fn send(&self, command: RuntimeCommand) -> bool {
        self.commands.send(command).is_ok()
    }

    pub fn snapshot(&self) -> RuntimeSnapshot {
        self.snapshots.borrow().clone()
    }

    pub fn drain_events(&mut self) -> Vec<RuntimeEvent> {
        let mut events = Vec::new();
        while let Ok(event) = self.events.try_recv() {
            events.push(event);
        }
        events
    }
}
