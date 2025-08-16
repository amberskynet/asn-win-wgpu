mod render_manager;
pub mod state;
pub mod state_error;
mod wgpu_context;

use asn_gui_core::TAsnRenderManager;
use asn_winit::WinitWindow;
use render_manager::RenderManager;

mod data;
pub mod rgba_handler;
mod texture;
pub mod wgpu_components;
mod wgpu_utils;

pub use state::State;
pub use state_error::StateError;

pub use rgba_handler::RgbaHandler;
pub use wgpu_components::wgpu_map::WgpuMap;
pub use wgpu_components::wgpu_mesh_color::WgpuQuad;
pub use wgpu_components::wgpu_mesh_textured::WgpuQuadTextured;

use crate::render_manager::WgpuFrameContext;

pub type GraphContext = render_manager::WgpuContext;
pub type FrameContext = render_manager::WgpuFrameContext;

pub fn get_manager() -> impl TAsnRenderManager<Window = WinitWindow, FrameContext = WgpuFrameContext>
{
    RenderManager::new()
}
