use asn_logger::trace;

const LOG_MODULE_NAME: &str = "wgpu_quad";

pub struct WgpuQuad {
    render_pipeline: wgpu::RenderPipeline,
}

impl WgpuQuad {
    pub fn new(render_pipeline: wgpu::RenderPipeline) -> Self {
        WgpuQuad { render_pipeline }
    }
    pub fn draw(&self, render_pass: &mut wgpu::RenderPass) {
        trace(LOG_MODULE_NAME, format!("draw").as_str());
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.draw(0..3, 0..1);
    }
}
