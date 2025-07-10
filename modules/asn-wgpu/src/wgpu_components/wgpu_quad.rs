use asn_logger::trace;
use crate::data::LOG_MODULE_NAME;

pub struct WgpuQuad {
    render_pipeline: wgpu::RenderPipeline,
}

impl WgpuQuad {
    pub fn new(render_pipeline: wgpu::RenderPipeline) -> Self {
        WgpuQuad { render_pipeline }
    }
    pub fn draw(&mut self) {
        trace(LOG_MODULE_NAME, format!("draw").as_str());
    }
}
