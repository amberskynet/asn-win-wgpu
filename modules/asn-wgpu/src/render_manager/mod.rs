mod asn_render_manager_impl;
mod asn_sutface_impl;
mod frame_context;
mod surface_state;
mod wgpu_context;

use surface_state::WgpuSurfaceState;

pub struct RenderManager {
    s: Option<WgpuSurfaceState>,
}

impl RenderManager {
    pub fn new() -> Self {
        RenderManager { s: None }
    }
}
