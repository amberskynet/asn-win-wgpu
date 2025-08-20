mod asn_render_manager_impl;
mod frame_context;
mod wgpu_context;

use std::sync::{Arc, Mutex};

use asn_gui_core::TAsnGuiHandler;

pub use frame_context::WgpuFrameContext;
pub use wgpu_context::WgpuContext;

pub trait GuiHandler:
    TAsnGuiHandler<
        GraphContext = wgpu_context::WgpuContext,
        FrameContext = frame_context::WgpuFrameContext,
    >
{
}

pub struct RenderManager<H>
where
    H: GuiHandler,
{
    s: Option<wgpu_context::WgpuContext>,
    h: Arc<Mutex<H>>,
}

impl<H> RenderManager<H>
where
    H: GuiHandler,
{
    pub fn new(h: Arc<Mutex<H>>) -> Self {
        RenderManager {
            s: None,
            h: h.clone(),
        }
    }
}
