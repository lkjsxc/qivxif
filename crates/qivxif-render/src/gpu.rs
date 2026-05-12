use crate::{CameraView, FrameStats, RenderScene, SmokeFrame};

pub struct GpuRenderer {
    _instance: wgpu::Instance,
    _adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    frames_rendered: u64,
}

impl GpuRenderer {
    pub async fn new() -> anyhow::Result<Self> {
        let instance = wgpu::Instance::default();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await?;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await?;
        Ok(Self {
            _instance: instance,
            _adapter: adapter,
            device,
            queue,
            frames_rendered: 0,
        })
    }

    pub fn render(&mut self, scene: &RenderScene, view: CameraView) -> FrameStats {
        let label = format!("qivxif native frame {}", self.frames_rendered + 1);
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("qivxif native evidence target"),
            size: wgpu::Extent3d {
                width: view.width.max(1),
                height: view.height.max(1),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let target = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some(&label),
            });
        {
            let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("qivxif native render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &target,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(clear_color(scene)),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });
        }
        self.queue.submit(Some(encoder.finish()));
        self.frames_rendered += 1;
        let evidence = SmokeFrame::render(&scene.cells, view.width as usize, view.height as usize);
        FrameStats {
            gpu_frames: self.frames_rendered,
            quads: scene.quads.len(),
            cells: scene.cells.len(),
            nonzero_pixels: evidence.nonzero_pixel_count(),
        }
    }
}

fn clear_color(scene: &RenderScene) -> wgpu::Color {
    let color = scene
        .quads
        .first()
        .map(|quad| quad.color)
        .unwrap_or_else(|| qivxif_assets::Color::new(0, 0, 0));
    wgpu::Color {
        r: f64::from(color.r) / 255.0,
        g: f64::from(color.g) / 255.0,
        b: f64::from(color.b) / 255.0,
        a: 1.0,
    }
}
