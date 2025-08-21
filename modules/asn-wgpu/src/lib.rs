extern crate asn_winit;

pub mod render_manager;
mod state_error;

mod data;

use std::sync::{Arc, Mutex};

use asn_gui_core::TAsnGuiHandler;
pub use state_error::StateError;

use crate::render_manager::RenderManager;

pub type GraphContext = render_manager::WgpuContext;
pub type FrameContext = render_manager::WgpuFrameContext;

pub trait WgpuGuiHandler:
    TAsnGuiHandler<
        GraphContext = render_manager::WgpuContext,
        FrameContext = render_manager::WgpuFrameContext,
    >
{
}

pub fn get_manager<H: WgpuGuiHandler>(h: Arc<Mutex<H>>) -> impl asn_winit::WinitRenderManager {
    RenderManager::new(h)
}
