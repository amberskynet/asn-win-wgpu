pub mod render_manager;
mod state_error;

mod data;

use std::sync::{Arc, Mutex};

use asn_gui_core::TAsnGuiHandler;
pub use state_error::StateError;

use crate::render_manager::RenderManager;

// reexport
pub use wgpu;

pub use render_manager::RenderManagerError;
pub use render_manager::WgpuFrameContext;
pub use render_manager::WgpuGraphContext;

pub trait WgpuGuiHandler:
    TAsnGuiHandler<
        GraphContext = render_manager::WgpuGraphContext,
        FrameContext = render_manager::WgpuFrameContext,
    >
{
}

// Blanket implementation
impl<T> WgpuGuiHandler for T where
    T: TAsnGuiHandler<
            GraphContext = render_manager::WgpuGraphContext,
            FrameContext = render_manager::WgpuFrameContext,
        >
{
}

// Выдаем на выход TAsnGuiHandler совместимый с WinitRenderManager
pub fn get_manager<H: WgpuGuiHandler>(h: Arc<Mutex<H>>) -> impl asn_winit::WinitRenderManager {
    RenderManager::new(h)
}
