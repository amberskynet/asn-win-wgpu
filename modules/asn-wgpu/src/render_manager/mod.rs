mod asn_render_manager_impl;
mod frame_context;
mod wgpu_context;

pub struct RenderManager {
    s: Option<wgpu_context::WgpuContext>,
}

impl RenderManager {
    pub fn new() -> Self {
        RenderManager { s: None }
    }
}
