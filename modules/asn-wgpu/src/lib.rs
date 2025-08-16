extern crate asn_winit;

mod render_manager;
mod state_error;

use asn_gui_core::TAsnRenderManager;
use render_manager::RenderManager;

mod data;
mod wgpu_utils;

pub use state_error::StateError;

use crate::render_manager::WgpuFrameContext;

pub type GraphContext = render_manager::WgpuContext;
pub type FrameContext = render_manager::WgpuFrameContext;

pub fn get_manager()
-> impl TAsnRenderManager<Window = asn_winit::WinitWindow, FrameContext = WgpuFrameContext> {
    RenderManager::new()
}
