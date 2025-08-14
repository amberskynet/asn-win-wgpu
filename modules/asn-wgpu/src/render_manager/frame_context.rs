use std::time::Instant;

use crate::StateError;

pub struct WgpuFrameContext {
    pub output: wgpu::SurfaceTexture,
    pub encoder: wgpu::CommandEncoder,
    pub view: wgpu::TextureView,
    pub frame_start: Instant,
}

impl WgpuFrameContext {
    pub fn new(
        surface: &wgpu::Surface<'static>,
        device: &wgpu::Device,
    ) -> Result<WgpuFrameContext, StateError> {
        let output = surface
            .get_current_texture()
            .map_err(|e| StateError::TextureError(e.to_string()))?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let frame_start = Instant::now();

        Ok(Self {
            output,
            encoder,
            view,
            frame_start,
        })
    }
}
