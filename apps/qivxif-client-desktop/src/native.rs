use crate::args::{E2eArgs, NativeArgs};
use crate::native_evidence::write_evidence;
use anyhow::Result;
use qivxif_client_core::{
    ClientConfig, ClientRuntimeHandle, RuntimeCommand, RuntimeEvent, RuntimeSnapshot, RuntimeStatus,
};
use qivxif_core::BlockPos;
use qivxif_input::{CameraState, PointerTarget};
use qivxif_render::{CameraView, GpuRenderer, RenderScene};
use std::time::{Duration, Instant};
use tokio::runtime::Runtime;
use winit::{
    event_loop::{ActiveEventLoop, EventLoop},
    window::Window,
};

pub(crate) const EDIT_POS: BlockPos = BlockPos { x: 1, y: 3, z: 1 };
pub(crate) const EDIT_BLOCK: u16 = 9;

pub fn run_interactive(args: NativeArgs, runtime: Runtime) -> Result<()> {
    let mut app = NativeApp::new(args, runtime, NativeMode::Interactive)?;
    EventLoop::new()?.run_app(&mut app)?;
    app.finish()
}

pub fn run_e2e(args: E2eArgs, runtime: Runtime) -> Result<()> {
    let mut app = NativeApp::new(args.native.clone(), runtime, NativeMode::E2e(args))?;
    EventLoop::new()?.run_app(&mut app)?;
    app.finish()
}

pub(crate) enum NativeMode {
    Interactive,
    E2e(E2eArgs),
}

pub(crate) enum E2eStage {
    WaitReady,
    WaitPlace,
    WaitRemove,
    Done,
}

pub(crate) struct NativeApp {
    pub(crate) runtime: Runtime,
    pub(crate) client: ClientRuntimeHandle,
    pub(crate) renderer: GpuRenderer,
    pub(crate) window: Option<Window>,
    pub(crate) camera: CameraState,
    pub(crate) target: PointerTarget,
    pub(crate) mode: NativeMode,
    pub(crate) stage: E2eStage,
    started: Instant,
    pub(crate) connected: bool,
    pub(crate) joined: bool,
    pub(crate) last_stats: qivxif_render::FrameStats,
    pub(crate) result: Result<()>,
}

impl NativeApp {
    fn new(args: NativeArgs, runtime: Runtime, mode: NativeMode) -> Result<Self> {
        let config = ClientConfig {
            addr: args.addr,
            server_name: args.server_name,
            tls_mode: args.tls,
        };
        let client =
            ClientRuntimeHandle::spawn(runtime.handle().clone(), config, args.player, args.radius);
        let renderer = runtime.block_on(GpuRenderer::new())?;
        Ok(Self {
            runtime,
            client,
            renderer,
            window: None,
            camera: CameraState::default(),
            target: PointerTarget { pos: EDIT_POS },
            mode,
            stage: E2eStage::WaitReady,
            started: Instant::now(),
            connected: false,
            joined: false,
            last_stats: qivxif_render::FrameStats {
                gpu_frames: 0,
                quads: 0,
                cells: 0,
                nonzero_pixels: 0,
            },
            result: Ok(()),
        })
    }

    fn finish(self) -> Result<()> {
        let _runtime = self.runtime;
        self.result
    }

    pub(crate) fn render_snapshot(&mut self) {
        let snapshot = self.client.snapshot();
        let scene = RenderScene::from_cells(snapshot.cached_cells.clone());
        self.last_stats = self.renderer.render(&scene, self.view());
    }

    fn view(&self) -> CameraView {
        CameraView {
            center_x: self.camera.center_x,
            center_z: self.camera.center_z,
            zoom: self.camera.zoom,
            width: 800,
            height: 600,
        }
    }

    pub(crate) fn drain_events(&mut self) {
        for event in self.client.drain_events() {
            match event {
                RuntimeEvent::Connected { .. } => self.connected = true,
                RuntimeEvent::Joined { .. } => self.joined = true,
                _ => {}
            }
        }
    }

    pub(crate) fn update_e2e(&mut self, event_loop: &ActiveEventLoop) {
        if self.started.elapsed() > Duration::from_secs(30) {
            self.result = Err(anyhow::anyhow!("desktop e2e timed out"));
            event_loop.exit();
            return;
        }
        let snapshot = self.client.snapshot();
        match self.stage {
            E2eStage::WaitReady if ready(&snapshot) => {
                self.render_snapshot();
                self.client.send(RuntimeCommand::Place {
                    pos: EDIT_POS,
                    block: EDIT_BLOCK,
                });
                self.stage = E2eStage::WaitPlace;
            }
            E2eStage::WaitPlace if has_ack(&snapshot, EDIT_BLOCK) && has_cell(&snapshot) => {
                self.render_snapshot();
                self.client.send(RuntimeCommand::Remove { pos: EDIT_POS });
                self.stage = E2eStage::WaitRemove;
            }
            E2eStage::WaitRemove
                if has_ack(&snapshot, qivxif_world::AIR) && !has_cell(&snapshot) =>
            {
                self.render_snapshot();
                let args = self.e2e_args().clone();
                self.result = write_evidence(&snapshot, self, &args);
                self.stage = E2eStage::Done;
                event_loop.exit();
            }
            _ => {}
        }
    }

    fn e2e_args(&self) -> &E2eArgs {
        match &self.mode {
            NativeMode::E2e(args) => args,
            NativeMode::Interactive => unreachable!("e2e args unavailable"),
        }
    }
}

fn ready(snapshot: &RuntimeSnapshot) -> bool {
    snapshot.status == RuntimeStatus::Ready && snapshot.chunks >= 9 && snapshot.cells > 0
}

pub(crate) fn has_ack(snapshot: &RuntimeSnapshot, block: u16) -> bool {
    snapshot
        .last_ack
        .as_ref()
        .is_some_and(|cell| cell.pos == EDIT_POS && cell.block == block)
}

pub(crate) fn has_cell(snapshot: &RuntimeSnapshot) -> bool {
    snapshot
        .cached_cells
        .iter()
        .any(|cell| cell.pos == EDIT_POS && cell.block == EDIT_BLOCK)
}
