mod asn_render_manager_impl;
mod frame_context;
mod wgpu_context;

use std::sync::{Arc, Mutex};

pub use frame_context::WgpuFrameContext;
pub use wgpu_context::WgpuGraphContext;

use crate::WgpuGuiHandler;

pub struct RenderManager<H>
where
    H: WgpuGuiHandler,
{
    s: Option<wgpu_context::WgpuGraphContext>,
    h: Arc<Mutex<H>>,
}

impl<H> RenderManager<H>
where
    H: WgpuGuiHandler,
{
    pub fn new(h: Arc<Mutex<H>>) -> Self {
        RenderManager {
            s: None,
            h: h.clone(),
        }
    }
}
