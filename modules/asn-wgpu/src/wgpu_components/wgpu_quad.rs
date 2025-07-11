use asn_logger::trace;
use crate::wgpu_utils::get_render_pipeline;

const LOG_MODULE_NAME: &str = "wgpu_quad";

pub struct WgpuQuad {
    render_pipeline: wgpu::RenderPipeline,
}

impl WgpuQuad {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        shader_source: &str,
    ) -> Self {
        let render_pipeline = get_render_pipeline(device, format, shader_source);
        WgpuQuad { render_pipeline }
    }
    pub fn draw(&self, render_pass: &mut wgpu::RenderPass) {
        trace(LOG_MODULE_NAME, format!("draw").as_str());
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.draw(0..3, 0..1);
    }
}
